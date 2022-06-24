/*
    Appellation: configuration
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use config::{Config, ConfigError, Environment, File};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub appellation: String,
    pub context: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
    pub application: Application,
    pub logger: Logger,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .set_default("application.appellation", "chaos")?
            .set_default("application.context", "dev")?
            .set_default("logger.level", "info")?;

        builder = builder.add_source(glob("**/*.config.*")
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

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings(application(appellation={}))", self.application.appellation)
    }
}