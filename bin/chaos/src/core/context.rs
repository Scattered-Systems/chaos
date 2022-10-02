/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use super::Settings;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(Settings::default())
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context(configuration={})", self.settings)
    }
}
