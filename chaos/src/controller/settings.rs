use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub mode: String,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub application: Application,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn new(mode: &str, name: &str) -> Result<Self, ConfigError> {
        let mode = "development";
        let name = "Application";
        let mut builder = Config::builder()
            .set_default("application.mode", mode.clone())?
            .set_default("application.name", name.clone())?
            .set_default("application.slug", name.to_lowercase().to_string())?
            .set_default("logger.level", "info")?
            .set_default("server.port", 8000)?;

        builder = builder.add_source(glob("**/*..config.*")
            .unwrap()
            .map(|path| File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );

        builder = builder.add_source(Environment::default().separator("__"));

        if let Ok(port) = std::env::var("PORT") {
            builder = builder
                .set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initializing application...")
    }
}