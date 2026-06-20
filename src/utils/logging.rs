use tracing_subscriber::{
    filter::EnvFilter,
    fmt,
    prelude::*,
};
use tracing_appender::rolling;

pub fn setup_logging(level: &str, log_to_file: bool) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    if log_to_file {
        let file_appender = rolling::daily("/var/log/speedcool", "speedcool.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::new().with_writer(non_blocking).json())
            .with(fmt::Layer::new().with_writer(std::io::stderr))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::new().with_writer(std::io::stderr))
            .init();
    }
}
