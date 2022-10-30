use std::sync::Arc;

/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::settings::Settings;
use scsys::{prelude::Id, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Session {
    pub id: Id,
    pub timestamp: Timestamp,
    pub data: Vec<String>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State {
    account: String,
    pub session: Session
}

impl State {
    pub fn new(account: String) -> Self {
        let session = Session::default();
        Self { account, session }
    }
    pub fn shared(&self) -> Arc<State> {
        Arc::new(self.clone())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
    
}
