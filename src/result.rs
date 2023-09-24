use std::fmt::Formatter;

use figment::error::Kind;
use git2::ErrorClass;
use git2::ErrorCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    BadParameter(String),
    BranchTemplateMalformed(String),
    Generic,
    MissingBranchParameter(String),
    NoCommitMessage,
    NoRepository,
    NoSelectedCommit,
    UnableToCreateBranch {
        branch_name: String,
        base_commit: String,
    },
    UnableToSelectBranch(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::BadParameter(m) => write!(f, "{m}"),
            Self::BranchTemplateMalformed(m)=>write!(f,"{m}"),
            Self::Generic => write!(f, "{}", "Generic"),
            Self::MissingBranchParameter(p)=>write!(f, "Missing parameter {p}"),
            Self::NoCommitMessage=>write!(f, "No commit message available for generating the branch name."),
            Self::NoRepository => write!(
                f,
                "Could not find a repository. Has `git init` been run?"
            ),
            Self::NoSelectedCommit => write!(
                f,
                "No currently selected commit. Are there any commits on this repository?"
            ),
            Self::UnableToCreateBranch {
                branch_name,
                base_commit,
            } => write!(
                f,
                "Could not create branch '{branch_name}' on commit {base_commit}.",
            ),
            Self::UnableToSelectBranch(b) => write!(f, "Could not switch to branch '{b}'.")
        }
    }
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        if e.class() == ErrorClass::Repository
            && e.code() == ErrorCode::NotFound
        {
            Self::NoRepository
        } else if e.class() == ErrorClass::Reference
            && e.code() == ErrorCode::UnbornBranch
        {
            Self::NoSelectedCommit
        } else {
            println!("Class = {:?}, Code = {:?}", e.class(), e.code());
            Self::Generic
        }
    }
}

impl From<figment::Error> for Error {
    fn from(e: figment::Error) -> Self {
        match e.kind {
            Kind::Message(m) => Self::BadParameter(m),

            // An invalid type: (actual, expected). See
            // [`serde::de::Error::invalid_type()`].
            //InvalidType(Actual, String)=>Self::BadParameter("InvalidType".to_string()),
            Kind::InvalidType(_, m) => {
                Self::BadParameter(format!("InvalidType: {}", m))
            }
            // An invalid value: (actual, expected). See
            // [`serde::de::Error::invalid_value()`].
            // InvalidValue(Actual, String)=>Self::BadParameter("InvalidType".to_string()),
            Kind::InvalidValue(_, _) => {
                Self::BadParameter("InvalidValue".to_string())
            }
            // Too many or too few items: (actual, expected). See
            // [`serde::de::Error::invalid_length()`].
            // InvalidLength(usize, String)=>Self::BadParameter("InvalidType".to_string()),
            Kind::InvalidLength(_, _) => {
                Self::BadParameter("InvalidLength".to_string())
            }

            // A variant with an unrecognized name: (actual, expected). See
            // [`serde::de::Error::unknown_variant()`].
            // UnknownVariant(String, &'static [&'static str])=>Self::BadParameter("InvalidType".to_string()),
            Kind::UnknownVariant(m, p) => Self::BadParameter(format!(
                "UnknownVariant: {}, expected one of {:?}",
                m, p
            )),
            // A field with an unrecognized name: (actual, expected). See
            // [`serde::de::Error::unknown_field()`].
            //UnknownField(String, &'static [&'static str])=>Self::BadParameter("InvalidType".to_string()),
            Kind::UnknownField(_, _) => {
                Self::BadParameter("UnknownField".to_string())
            }
            // A field was missing: (name). See [`serde::de::Error::missing_field()`].
            //MissingField(Cow<'static, str>)=>Self::BadParameter("InvalidType".to_string()),
            Kind::MissingField(m) => {
                Self::BadParameter(format!("MissingField: {}", m))
            }
            // A field appeared more than once: (name). See
            // [`serde::de::Error::duplicate_field()`].
            // DuplicateField(&'static str)=>Self::BadParameter("InvalidType".to_string()),
            Kind::DuplicateField(_) => {
                Self::BadParameter("DuplicateField".to_string())
            }

            // The `isize` was not in range of any known sized signed integer.
            // ISizeOutOfRange(isize)=>Self::BadParameter("InvalidType".to_string()),
            Kind::ISizeOutOfRange(_) => {
                Self::BadParameter("ISizeOutOfRange".to_string())
            }
            // The `usize` was not in range of any known sized unsigned integer.
            // USizeOutOfRange(usize)=>Self::BadParameter("InvalidType".to_string()),
            Kind::USizeOutOfRange(_) => {
                Self::BadParameter("USizeOutOfRange".to_string())
            }

            // The serializer or deserializer does not support the `Actual` type.
            // Unsupported(Actual)=>Self::BadParameter("InvalidType".to_string()),
            Kind::Unsupported(_) => {
                Self::BadParameter("Unsupported".to_string())
            }

            // The type `.0` cannot be used for keys, need a `.1`.
            // UnsupportedKey(Actual, Cow<'static, str>)=>Self::BadParameter("InvalidType".to_string()),
            Kind::UnsupportedKey(_, _) => {
                Self::BadParameter("UnsupportedKey".to_string())
            }
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Message {
    Empty,
}
