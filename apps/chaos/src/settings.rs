use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub mode: bool,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Peer {
    pub port: u16,
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View the server locally at http://localhost:{}", self.port)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub application: Application,
    pub logger: Logger,
    pub peer: Peer,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let name = "Application";
        let mut builder = Config::builder()
            .set_default("application.mode", false)?
            .set_default("application.name", name.clone())?
            .set_default("application.slug", name.to_lowercase())?
            .set_default("logger.level", "info")?
            .set_default("server.port", 8000)?;

        builder = builder.add_source(glob("**/*.contracts.*")
            .unwrap()
            .map(|path| File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );

        builder = builder.add_source(Environment::default().separator("__"));
        if let Ok(dev_mode) = std::env::var("DEV_MODE") {
            builder = builder
                .set_override("application.mode", dev_mode)?;
        }
        if let Ok(port) = std::env::var("PORT") {
            builder = builder
                .set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(mode={}, name={}, slug={})", self.application.mode, self.application.name, self.application.slug)
    }
}