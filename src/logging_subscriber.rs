use std::collections::HashMap;
use std::fmt;
use std::ops::DerefMut;
use std::path::PathBuf;

use console::Style;
use log::Record;
use tracing::{Event, Level};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

use crate::{LevelOutput, LoggingSubscriberBuilder, LoggingSubscriberLayer, LoggingWriter, LOGGING_WRITER};

#[derive(Default)]
struct ToStringVisitor<'a>(HashMap<&'a str, String>);

impl fmt::Display for ToStringVisitor<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.iter().try_for_each(|(_k, v)| -> fmt::Result { write!(f, "{}", v) })
	}
}

impl<'a> tracing::field::Visit for ToStringVisitor<'a> {
	fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
		self.0.insert(field.name(), format_args!("{}", value).to_string());
	}

	fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
		self.0.insert(field.name(), format_args!("{}", value).to_string());
	}

	fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
		self.0.insert(field.name(), format_args!("{}", value).to_string());
	}

	fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
		self.0.insert(field.name(), format_args!("{}", value).to_string());
	}

	fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
		self.0.insert(field.name(), format_args!("{}", value).to_string());
	}

	fn record_error(&mut self, field: &tracing::field::Field, value: &(dyn std::error::Error + 'static)) {
		self.0.insert(field.name(), format_args!("{}", value).to_string());
	}

	fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
		self.0.insert(field.name(), format_args!("{:?}", value).to_string());
	}
}

impl Default for LoggingSubscriberBuilder {
	fn default() -> Self {
		LoggingSubscriberBuilder {
			display_line_number: false,
			display_level: true,
			display_time: true,
			display_target: false,
			display_filename: false,
			default_style: Style::new().white(),
			date_time_style: Style::default().dim(),
			level_style_error: Style::new().red().bold(),
			level_style_warn: Style::new().magenta().bold().bright(),
			level_style_debug: Style::new().blue().bold(),
			level_style_trace: Style::new().black().bold(),
			level_style_info: Style::new().green().bright().bold(),
			style_error: None,
			style_warn: None,
			style_info: None,
			style_debug: None,
			style_trace: None,
			min_level: LevelFilter::DEBUG,
			separator: " ".to_string(),
			timestamp_format: "%H:%M:%S%.3f".to_string(),
			format_level: LevelOutput::Long,
		}
	}
}

impl From<LoggingSubscriberBuilder> for LoggingWriter {
	fn from(value: LoggingSubscriberBuilder) -> Self {
		let mut logging = LoggingWriter::default();
		logging.enabled = true;
		logging.level = value.min_level;
		logging.default_style = value.default_style;
		logging.style_error = value.style_error;
		logging.style_warn = value.style_warn;
		logging.style_debug = value.style_debug;
		logging.style_trace = value.style_trace;
		logging.style_info = value.style_info;
		logging.level_style_error = value.level_style_error;
		logging.level_style_warn = value.level_style_warn;
		logging.level_style_debug = value.level_style_debug;
		logging.level_style_trace = value.level_style_trace;
		logging.level_style_info = value.level_style_info;
		logging.separator = value.separator;
		logging.timestamp_format = value.timestamp_format;
		logging.format_level = value.format_level;
		logging.display_line_number = value.display_line_number;
		logging.display_level = value.display_level;
		logging.display_target = value.display_target;
		logging.display_filename = value.display_filename;
		logging.display_time = value.display_time;
		logging.date_time_style = value.date_time_style;
		logging
	}
}

#[allow(dead_code)]
impl LoggingSubscriberBuilder {
	pub fn build(self) -> LoggingSubscriberLayer {
		if let Ok(mut item) = LOGGING_WRITER.lock() {
			*item = self.into();
		}

		let subscriber = LoggingSubscriberLayer {};
		subscriber
	}

	pub fn with_min_level(mut self, value: LevelFilter) -> Self {
		self.min_level = value;
		self
	}

	pub fn with_separator(mut self, value: String) -> Self {
		self.separator = value;
		self
	}
	pub fn with_timestamp_format(mut self, value: String) -> Self {
		self.timestamp_format = value;
		self
	}
	pub fn with_format_level(mut self, value: LevelOutput) -> Self {
		self.format_level = value;
		self
	}

	pub fn with_default_style<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.default_style = value.into();
		self
	}

	pub fn with_date_time_style<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.date_time_style = value.into();
		self
	}

	pub fn with_level_style_error<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.level_style_error = value.into();
		self
	}
	pub fn with_level_style_warn<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.level_style_warn = value.into();
		self
	}
	pub fn with_level_style_debug<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.level_style_debug = value.into();
		self
	}
	pub fn with_level_style_trace<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.level_style_trace = value.into();
		self
	}
	pub fn with_level_style_info<S>(mut self, value: S) -> Self
	where
		S: Into<Style>,
	{
		self.level_style_info = value.into();
		self
	}
	pub fn with_style_error<S>(mut self, value: Option<S>) -> Self
	where
		S: Into<Style>,
	{
		self.style_error = match value {
			Some(value) => Some(value.into()),
			None => None,
		};
		self
	}
	pub fn with_style_warn<S>(mut self, value: Option<S>) -> Self
	where
		S: Into<Style>,
	{
		self.style_warn = match value {
			Some(value) => Some(value.into()),
			None => None,
		};
		self
	}
	pub fn with_style_info<S>(mut self, value: Option<S>) -> Self
	where
		S: Into<Style>,
	{
		self.style_info = match value {
			Some(value) => Some(value.into()),
			None => None,
		};
		self
	}
	pub fn with_style_debug<S>(mut self, value: Option<S>) -> Self
	where
		S: Into<Style>,
	{
		self.style_debug = match value {
			Some(value) => Some(value.into()),
			None => None,
		};
		self
	}
	pub fn with_style_trace<S>(mut self, value: Option<S>) -> Self
	where
		S: Into<Style>,
	{
		self.style_trace = match value {
			Some(value) => Some(value.into()),
			None => None,
		};
		self
	}

	pub fn with_line_number(mut self, display_line_number: bool) -> Self {
		self.display_line_number = display_line_number;
		self
	}

	pub fn with_level(mut self, display_level: bool) -> Self {
		self.display_level = display_level;
		self
	}

	pub fn with_time(mut self, display_time: bool) -> Self {
		self.display_time = display_time;
		self
	}

	pub fn with_target(mut self, display_target: bool) -> Self {
		self.display_target = display_target;
		self
	}

	pub fn with_file(mut self, display_filename: bool) -> Self {
		self.display_filename = display_filename;
		self
	}
}

impl<S> Layer<S> for LoggingSubscriberLayer
where
	S: tracing::Subscriber,
{
	fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
		let mut visitor = ToStringVisitor::default();
		event.record(&mut visitor);

		let level = match *event.metadata().level() {
			Level::ERROR => log::Level::Error,
			Level::WARN => log::Level::Warn,
			Level::INFO => log::Level::Info,
			Level::DEBUG => log::Level::Debug,
			Level::TRACE => log::Level::Trace,
		};

		let buf = match event.metadata().file() {
			None => PathBuf::new(),
			Some(file) => PathBuf::from(file),
		};

		let filename = buf.file_name().map(|s| s.to_str().unwrap_or("?"));

		let _ = LOGGING_WRITER.lock().unwrap().deref_mut().log(
			&Record::builder()
				.args(format_args!("{}", visitor))
				.level(level.into())
				.target(event.metadata().target())
				.file(filename)
				.line(event.metadata().line())
				.module_path(event.metadata().module_path())
				.build(),
		);
	}
}
