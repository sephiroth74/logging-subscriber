use console::Style;
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::Write;
use std::ops::DerefMut;

use log::Record;
use tracing_log::AsLog;
use tracing_subscriber::fmt::MakeWriter;

use crate::{BlockingWriter, LevelOutput, LoggingWriter, LOGGING_WRITER};

impl Default for LoggingWriter {
	fn default() -> Self {
		LoggingWriter {
			enabled: true,
			level: tracing::Level::DEBUG,
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

			timestamp_format: "%Y-%m-%dT%H:%M:%S%.3f".to_string(),
			separator: String::from(" "),
			format_level: LevelOutput::Abbreviated,
			display_level: true,
			display_time: true,
			display_target: false,
			display_filename: false,
			display_line_number: false,
		}
	}
}

impl Write for LoggingWriter {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		if self.enabled {
			io::stdout().write(buf)
		} else {
			Ok(0)
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		io::stdout().flush()
	}
}

impl LoggingWriter {
	pub fn log(&mut self, record: &Record) -> io::Result<usize> {
		if self.level.as_log() >= record.level() {
			self.write(self.format_event(record).as_bytes())
		} else {
			Ok(0)
		}
	}

	fn format_event(&self, evt: &Record) -> String {
		let mut output = String::new();
		let mut default_style = self.default_style.clone();

		let (col_style, lev_long, lev_abbr) = match evt.level() {
			log::Level::Error => {
				default_style = self.style_error.clone().unwrap_or(default_style);
				(self.level_style_error.clone(), "ERROR", "E")
			}

			log::Level::Warn => {
				default_style = self.style_warn.clone().unwrap_or(default_style);
				(self.level_style_warn.clone(), "WARN ", "W")
			}
			log::Level::Info => {
				default_style = self.style_info.clone().unwrap_or(default_style);
				(self.level_style_info.clone(), "INFO ", "I")
			}
			log::Level::Debug => {
				default_style = self.style_debug.clone().unwrap_or(default_style);
				(self.level_style_debug.clone(), "DEBUG", "D")
			}
			log::Level::Trace => {
				default_style = self.style_trace.clone().unwrap_or(default_style);
				(self.level_style_trace.clone(), "TRACE", "T")
			}
		};

		if self.display_time {
			let _ = write!(
				&mut output,
				"{}",
				self.date_time_style
					.apply_to(chrono::Local::now().format(&self.timestamp_format).to_string())
			);
			let _ = write!(&mut output, "{}", self.default_style.apply_to(&self.separator));
		}

		match self.format_level {
			LevelOutput::Abbreviated => {
				let s = format!("{: ^3}", lev_abbr);
				let _ = write!(&mut output, "{}", col_style.apply_to(s));
				let _ = write!(&mut output, "{}", self.default_style.apply_to(&self.separator));
			}
			LevelOutput::Long => {
				let _ = write!(&mut output, "{}", col_style.apply_to(lev_long));
				let _ = write!(&mut output, "{}", self.default_style.apply_to(&self.separator));
			}
			_ => {}
		}

		let mut target_written = false;
		let mut file_written: bool = false;
		let mut line_written: bool = false;

		if self.display_target {
			let _ = write!(&mut output, "{}", self.default_style.apply_to(evt.target()));
			target_written = true;
		}

		if self.display_filename {
			if target_written {
				let _ = write!(&mut output, "{}", self.default_style.apply_to(&self.separator));
			}

			let _ = write!(&mut output, "{}", self.default_style.apply_to("<"));
			let _ = write!(&mut output, "{}", self.default_style.apply_to(evt.file().unwrap_or("?")));
			file_written = true;
		}

		if self.display_line_number {
			if file_written {
				let _ = write!(&mut output, "{}", self.default_style.apply_to(":"));
			}
			let _ = write!(
				&mut output,
				"{}",
				self.default_style.apply_to(evt.line().unwrap_or(0).to_string())
			);
			let _ = write!(&mut output, "{}", self.default_style.apply_to(">"));
			file_written = true;
			line_written = true;
		}

		if file_written && !line_written {
			let _ = write!(&mut output, "{}", self.default_style.apply_to(">"));
		}

		if file_written || target_written {
			let _ = write!(&mut output, "{}", self.default_style.apply_to(": "));
		}

		let _ = write!(&mut output, "{}\n", default_style.apply_to(format!("{}", evt.args())));
		output
	}
}

impl Write for BlockingWriter {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		LOGGING_WRITER.lock().unwrap().deref_mut().write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		LOGGING_WRITER.lock().unwrap().deref_mut().flush()
	}
}

impl<'a> MakeWriter<'a> for BlockingWriter {
	type Writer = BlockingWriter;

	fn make_writer(&'a self) -> Self::Writer {
		self.clone()
	}
}
