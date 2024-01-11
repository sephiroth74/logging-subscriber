#[cfg(test)]
mod tests {
	use termbg::Theme;
	use tracing::{debug, error, info, info_span, subscriber, trace, warn};
	use tracing_subscriber::filter::LevelFilter;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::Registry;

	use crate::LoggingSubscriberBuilder;

	#[test]
	fn test_simple() {
		let registry = Registry::default();
		let term_subscriber = LoggingSubscriberBuilder::default().with_min_level(LevelFilter::TRACE).build();
		let subscriber = registry.with(term_subscriber);
		subscriber::set_global_default(subscriber).unwrap();

		info!("Logging set!");
		debug!("Debug message");
		trace!("Debug message");
		warn!("Debug message");
		error!("Debug message");

		let span = info_span!("my_great_span");
		{
			let _enter = span.enter();
			info!("i'm in the span!");
		}
		info!("i'm outside the span!")
	}

	#[test]
	fn test_term_color() {
		println!("Check terminal background color");
		let timeout = std::time::Duration::from_millis(100);
		let _term = termbg::terminal();
		let theme = termbg::theme(timeout).unwrap_or(Theme::Dark);
		let dark_theme = terminal_light::luma().map_or(false, |luma| luma <= 0.5);

		println!("theme: {:?}", theme);
		println!("is dark: {:?}", dark_theme);
	}
}
