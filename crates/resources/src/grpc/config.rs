use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Config {
    pub(crate) address: (u8, u8, u8, u8),
    pub(crate) port: u16,
}

impl Config {
    pub fn new(address: (u8, u8, u8, u8), port: u16) -> Self {
        tracing_subscriber::registry()
            .with(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("error,resources=debug")),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
        Self { address, port }
    }
}
