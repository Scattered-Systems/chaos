/*
   Appellation: application <module>
   Creator: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
        ... Summary ...

*/
use super::cli::Cli;

use scsys::Timestamp;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub timestamp: Timestamp,
}

impl Application {
    pub fn new() -> Self {
        let timestamp = Timestamp::default();
        Self { timestamp }
    }
    pub fn cli(&self) -> Cli {
        Cli::default()
    }
    pub async fn run(&self) -> scsys::BoxResult {
        let cli = self.cli();

        println!("{:#?}", cli);

        Ok(())
    }
}
