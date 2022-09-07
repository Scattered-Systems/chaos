/*
    Appellation: ipfs <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IpfsActor;

impl IpfsActor {
    pub fn new() -> Self {
        Self
    }
    pub fn in_memory(&self) -> crate::Ipfs {
        todo!()
    }
}
