use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String
}

impl Logger {
    pub fn constructor(configuration: Settings) {
        if std::env::var_os("RUST_LOG").is_none() {
            let level = configuration.logger.level.as_str();
            let env = format!("dapi={},tower_http=debug", level);

            std::env::set_var("RUST_LOG", env);
        }
        tracing_subscriber::fmt::init()
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger(level={})", self.level)
    }
}