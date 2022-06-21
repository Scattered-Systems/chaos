/*
    Appellation: actor
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Actor {
    pub appellation: String,
    pub description: String

}

impl Actor {
    pub fn new(appellation: String, description: String) -> Self {
        Self {
            appellation, description
        }
    }
}

impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(appellation={})", self.appellation)
    }
}