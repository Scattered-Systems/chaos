/*
    Appellation: application <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::api::Api;
use crate::{Context, Settings};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Backend {
    pub context: Context,
}

impl Backend {
    pub fn new() -> Self {
        println!("Configuring the application...");
        let settings = Settings::default();
        println!("{}", settings.clone());
        let context = Context::new(settings.clone());
        Self { context }
    }
    pub fn from(settings: Settings) -> Self {
        Self {
            context: Context::new(settings),
        }
    }
    pub fn api(&self) -> Api {
        Api::new(self.context.clone())
    }
    pub fn with_logging(&self) -> &Self {
        self.context.settings.logger.setup();
        self
    }
    pub async fn run(&self) -> scsys::BoxResult {
        println!("{}", self.context.settings.server.clone());
        self.api().run().await.expect("Interface Error");
        Ok(())
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::from(Settings::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        let actual = f(4, 4);
        let expected: usize = 8;
        assert_eq!(actual, expected)
    }
}
