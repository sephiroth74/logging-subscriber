# logging-subscriber

[![crates.io](https://img.shields.io/crates/v/logging-subscriber.svg)](https://crates.io/crates/logging-subscriber)
[![ci](https://github.com/sephiroth74/logging-subscriber/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/sephiroth74/logging-subscriber/actions/workflows/rust.yml)

### Usage:
```rust
use std::sync::{LazyLock, MutexGuard, PoisonError, RwLock};

use console::Style;
use logging_subscriber;
use logging_subscriber::LoggingWriter;
use tracing::level_filters::LevelFilter;
use tracing::subscriber;
use tracing_log::AsLog;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, Registry};

pub static LOGGING_LEVEL: LazyLock<RwLock<LevelFilter>> = LazyLock::new(|| RwLock::new(LevelFilter::TRACE));

///
/// Initialize logging with the given filter
pub fn init_logging(level_filter: LevelFilter) {
	let registry = Registry::default();
	tui_logger::init_logger(LevelFilter::OFF.as_log()).unwrap();
	tui_logger::set_default_level(LevelFilter::OFF.as_log());

	let timestamp_format = "%H:%M:%S".to_string();

	let term_subscriber = logging_subscriber::LoggingSubscriberBuilder::default()
		.with_time(true)
		.with_timestamp_format(timestamp_format)
		.with_level(true)
		.with_target(false)
		.with_file(false)
		.with_line_number(false)
		.with_min_level(level_filter)
		.with_format_level(logging_subscriber::LevelOutput::Long)
		.with_default_style(Style::default().dim())
		.with_level_style_warn(Style::new().color256(220).bold())
		.with_level_style_trace(Style::new().magenta().bold())
		.with_date_time_style(Style::new().white())
		.build();

	let filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::TRACE.into())
		.from_env()
		.unwrap();

	let subscriber = registry
		.with(filter)
		.with(term_subscriber)
		.with(tui_logger::tracing_subscriber_layer());
	subscriber::set_global_default(subscriber).unwrap();

	*LOGGING_LEVEL.write().unwrap() = level_filter;
}

/// Pause logging of tracing events
pub fn pause_logging() {
	let _ = logging_subscriber::set_enabled(false);
	tui_logger::set_default_level(LevelFilter::TRACE.as_log());
}
/// Returns true if logging is enabled
pub fn is_enabled() -> Result<bool, PoisonError<MutexGuard<'static, LoggingWriter>>> {
	logging_subscriber::is_enabled()
}

// Resume logging of tracing events
pub fn resume_logging() {
	let _ = logging_subscriber::set_enabled(true);
	tui_logger::set_default_level(tracing::level_filters::LevelFilter::OFF.as_log());
}

/// Returns the current logging level
pub fn get_logging_level() -> LevelFilter {
	match LOGGING_LEVEL.read() {
		Ok(l) => *l,
		Err(_) => LevelFilter::OFF,
	}
}

/// Returns true if the current logging level is set to TRACE
pub fn is_logging_verbose() -> bool {
	get_logging_level() == LevelFilter::TRACE
}
```
