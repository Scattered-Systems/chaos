/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::api::Api;
use crate::core::{Context, Settings};

use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Chaos {
    pub context: Context,
}

impl Chaos {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
    pub fn context(&self) -> Context {
        self.context.clone()
    }
    pub fn get_settings(&self) -> Settings {
        self.context.settings.clone()
    }
    pub fn api(&self) -> Api {
        Api::new(self.context())
    }
    pub fn with_logging(&self) -> &Self {
        self.context.settings.logger.setup();
        self
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        println!("{}", self.context.settings.server.clone());
        match self.api().run().await {
            Ok(_) => {}
            Err(_) => panic!("{:?}", scsys::Error::Default),
        };
        Ok(self)
    }
}

impl std::convert::From<Settings> for Chaos {
    fn from(settings: Settings) -> Self {
        Self::new(Context::new(settings))
    }
}

impl Default for Chaos {
    fn default() -> Self {
        Self::new(Context::default())
    }
}
