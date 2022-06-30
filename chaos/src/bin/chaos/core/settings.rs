/*
    Appellation: settings
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use config::{Config, ConfigError, Environment, File};

type Builder<T = config::builder::DefaultState> = config::ConfigBuilder<T>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub context: String,
    pub name: String,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub application: Application,
    pub logger: Logger,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .set_default("application.context", "development")?
            .set_default("application.name", "chaos")?
            .set_default("logger.level", "info")?;

        builder = collect_config_files(builder, "**/*.config.*");

        builder = builder.add_source(Environment::default().separator("__"));
        builder.build()?.try_deserialize()
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(context={}, name={})", self.context, self.name)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings(application={})", self.application)
    }
}

pub fn collect_config_files(builder: Builder, pattern: &str) -> Builder {
    builder.add_source(
        glob::glob(pattern)
            .unwrap()
            .map(|path| File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}