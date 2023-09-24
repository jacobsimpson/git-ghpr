use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

const TARGET: &str = env!("CARGO_PKG_NAME");

pub fn init(verbose_level: u8) {
    let filter_layer = match verbose_level {
        0 => Targets::new().with_default(LevelFilter::OFF),
        1 => Targets::new().with_target(TARGET, LevelFilter::ERROR),
        2 => Targets::new().with_target(TARGET, LevelFilter::WARN),
        3 => Targets::new().with_target(TARGET, LevelFilter::INFO),
        4 => Targets::new().with_target(TARGET, LevelFilter::DEBUG),
        5 => Targets::new().with_target(TARGET, LevelFilter::TRACE),
        6 => Targets::new().with_default(LevelFilter::ERROR),
        7 => Targets::new().with_default(LevelFilter::WARN),
        8 => Targets::new().with_default(LevelFilter::INFO),
        9 => Targets::new().with_default(LevelFilter::DEBUG),
        _ => Targets::new().with_default(LevelFilter::TRACE),
    };

    let fmt_layer = fmt::layer()
        .with_level(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_ids(false)
        .compact();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
