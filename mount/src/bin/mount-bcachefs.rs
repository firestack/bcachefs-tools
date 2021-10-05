fn main() {
	// convert existing log statements to tracing events
	// tracing_log::LogTracer::init().expect("logtracer init failed!");
	// format tracing log data to env_logger like stdout
	tracing_subscriber::fmt::init();

	if let Err(e) = bcachefs_mount::main_inner() {
		tracing::error!(err = ?e);
	}
}

#[cfg(test)]
mod test {
	use insta::assert_debug_snapshot;
	#[test]
	fn snapshot_testing() {
		insta::assert_debug_snapshot!();
	}
}
