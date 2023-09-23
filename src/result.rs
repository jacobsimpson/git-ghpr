use std::fmt::Formatter;

use git2::ErrorClass;
use git2::ErrorCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    NoRepository,
    Generic,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::NoRepository => write!(
                f,
                "Could not find a repository. Has `git init` been run?"
            ),
            Self::Generic => write!(f, "{}", "Generic"),
        }
    }
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        if e.class() == ErrorClass::Repository
            && e.code() == ErrorCode::NotFound
        {
            Self::NoRepository
        } else {
            Self::Generic
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Message {
    Empty,
}
