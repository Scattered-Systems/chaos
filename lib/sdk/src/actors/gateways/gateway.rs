/*
    Appellation: gateway <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Event {
    Fetch,
    Sign,
    #[default]
    Idle
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum GatewayState {
    #[default]
    Idle
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Gateway {
    pub state: GatewayState
}

impl Gateway {
    pub fn new(state: Option<GatewayState>) -> Self {
        let state = match state {
            Some(v) => v,
            None => GatewayState::default()
        };

        Self { state }
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new(None)
    }
}
