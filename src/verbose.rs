use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub fn init(verbose_level: u8) {
    let fmt_layer = fmt::layer()
        .with_level(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_ids(false)
        .compact();

    let filter_layer = match verbose_level {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
