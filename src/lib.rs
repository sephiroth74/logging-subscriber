use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use console::{Style, StyledObject};
use lazy_static::lazy_static;
use tracing_subscriber::filter::LevelFilter;

mod logging_subscriber;
mod logging_writer;
mod prelude;
mod test;

lazy_static! {
	pub static ref LOGGING_WRITER: Arc<Mutex<LoggingWriter>> = Arc::new(Mutex::new(LoggingWriter::default()));
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BlockingWriter {}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LoggingWriter {
	pub(crate) enabled: bool,
	pub(crate) level: tracing::metadata::LevelFilter,

	default_style: Style,

	date_time_style: Style,

	style_error: Option<Style>,
	style_warn: Option<Style>,
	style_debug: Option<Style>,
	style_trace: Option<Style>,
	style_info: Option<Style>,

	level_style_error: Style,
	level_style_warn: Style,
	level_style_debug: Style,
	level_style_trace: Style,
	level_style_info: Style,

	separator: String,
	timestamp_format: String,
	format_level: LevelOutput,

	display_line_number: bool,
	display_level: bool,
	display_target: bool,
	display_filename: bool,
	display_time: bool,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum LevelOutput {
	Abbreviated,
	Long,
	None,
}

#[derive(Debug, Clone)]
pub struct LoggingSubscriberLayer;

#[derive(Debug, Clone)]
pub struct LoggingSubscriberBuilder {
	pub display_line_number: bool,
	pub display_level: bool,
	pub display_time: bool,
	pub display_target: bool,
	pub display_filename: bool,

	default_style: Style,
	date_time_style: Style,

	level_style_error: Style,
	level_style_warn: Style,
	level_style_debug: Style,
	level_style_trace: Style,
	level_style_info: Style,

	style_error: Option<Style>,
	style_warn: Option<Style>,
	style_info: Option<Style>,
	style_debug: Option<Style>,
	style_trace: Option<Style>,

	min_level: tracing::metadata::LevelFilter,
	separator: String,
	timestamp_format: String,
	format_level: LevelOutput,
}

#[derive(Debug, Default, Clone)]
pub struct AdaptiveStyle {
	pub(crate) light: console::Style,
	pub(crate) dark: console::Style,
}

impl AdaptiveStyle {
	pub fn new(light: console::Style, dark: console::Style) -> Self {
		AdaptiveStyle { light, dark }
	}

	pub fn from(style: console::Style) -> Self {
		AdaptiveStyle {
			light: style.clone(),
			dark: style.clone(),
		}
	}

	fn is_dark_background() -> bool {
		terminal_light::luma().map_or(false, |luma| luma <= 0.5)
	}

	#[must_use]
	pub fn paint<D>(&self, val: D) -> StyledObject<D> {
		if AdaptiveStyle::is_dark_background() {
			self.dark.apply_to(val)
		} else {
			self.dark.apply_to(val)
		}
	}
}

impl From<console::Style> for AdaptiveStyle {
	fn from(value: console::Style) -> Self {
		AdaptiveStyle::from(value)
	}
}

impl From<AdaptiveStyle> for console::Style {
	fn from(value: AdaptiveStyle) -> Self {
		let dark_theme = terminal_light::luma().map_or(false, |luma| luma <= 0.5);
		if dark_theme {
			value.dark
		} else {
			value.light
		}
	}
}

#[allow(dead_code)]
pub fn set_enabled(value: bool) -> Result<(), PoisonError<MutexGuard<'static, LoggingWriter>>> {
	match LOGGING_WRITER.lock() {
		Ok(mut item) => {
			item.enabled = value;
			Ok(())
		}
		Err(err) => Err(err),
	}
}

#[allow(dead_code)]
pub fn set_level(value: LevelFilter) -> Result<(), PoisonError<MutexGuard<'static, LoggingWriter>>> {
	match LOGGING_WRITER.lock() {
		Ok(mut item) => {
			item.level = value;
			Ok(())
		}
		Err(err) => Err(err),
	}
}
