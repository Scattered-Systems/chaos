/*
    Appellation: context
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Context {
    pub configuration: Settings
}

impl Context {
    pub fn constructor(configuration: Settings) -> Self {
        Self {
            configuration
        }
    }
}

