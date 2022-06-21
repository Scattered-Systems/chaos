/*
    Appellation: context
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use crate::Configuration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Context {
    pub configuration: Configuration
}

impl Context {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration,

        }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context(configuration={})", self.configuration)
    }
}