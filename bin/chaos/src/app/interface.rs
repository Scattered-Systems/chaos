/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{Context, Settings};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Chaos {
    pub context: Context
}

impl Chaos {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub fn get_context(&self) -> Context {
        self.context.clone()
    }

    pub fn get_settings(&self) -> Settings {
        self.context.settings.clone()
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
