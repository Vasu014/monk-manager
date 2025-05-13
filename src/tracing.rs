use anyhow::Result;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::UtcTime},
    prelude::*,
    EnvFilter,
};

pub fn init_tracing() -> Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(true)
        .with_level(true)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(std::io::stderr)
        .with_span_events(FmtSpan::CLOSE);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("monk_manager=info"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    Ok(())
}

pub fn set_log_level(level: Level) {
    let filter_layer = EnvFilter::try_new(format!("monk_manager={}", level.as_str())).unwrap();
    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_initialization() {
        let result = init_tracing();
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_level_setting() {
        set_log_level(Level::DEBUG);
        // No assertion needed as this is just testing that it doesn't panic
    }
} 