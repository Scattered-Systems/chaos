use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub appellation: String,
    pub description: String

}

impl Application {
    pub fn new(appellation: String, description: String) -> Self {
        Self {
            appellation, description
        }
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application(appellation={})", self.appellation)
    }
}