use crate::settings::Settings;
use serde::{Deserialize, Serialize};

pub trait Interface {
    type Configure;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings
}

pub impl Interface for Context {
    type Configure = Settings;
}