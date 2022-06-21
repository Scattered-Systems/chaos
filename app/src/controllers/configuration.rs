use config::{Config, ConfigError, File};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
    pub application: crate::Application,
    pub logger: crate::Logger
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let project = "Chaos";
        let mut builder = Config::builder()
            .set_default("application.appellation", project.to_lowercase())?
            .set_default("logger.level", "debug")?;

        builder = builder.add_source(glob("**/*.config.*")
            .unwrap()
            .map(|path| File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
        );
        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings for {}", self.application.appellation)
    }
}