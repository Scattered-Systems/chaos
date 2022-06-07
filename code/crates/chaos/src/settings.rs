use std::{collections::HashMap, net::SocketAddr};

use serde::{Deserialize, Serialize};

pub struct Application {
    pub authorizations: bool,

    pub name: String,
}

pub struct Registry {
    pub label: String,
    pub endpoint: String,
    pub private: HashMap<String, String>,
}

pub struct Settings;

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initializing application...")
    }
}