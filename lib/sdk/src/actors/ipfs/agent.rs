/*
    Appellation: ipfs <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

use ipfs::IpfsOptions;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]

pub enum EngineState {
    Off,
    On,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Engine {
    pub state: EngineState
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IpfsAgentMode {
    InMemory,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IpfsAgent;

impl IpfsAgent {
    pub fn new() -> Self {
        Self
    }
    pub fn in_memory(&self) -> IpfsOptions {
        IpfsOptions::inmemory_with_generated_keys()
    }
}


#[cfg(test)]
mod tests {
    use super::IpfsAgent;

    #[test]
    fn test_default_agent() {
        let a = IpfsAgent::new();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
