/*
    Appellation: interface
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
use chaos_sdk::CommandLineInterface;
use clap::Parser;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct App {
    pub name: String,
}

impl App {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl CommandLineInterface for App {
    type Args = crate::CliOptions;

    fn client(&self) -> Self::Args {
        Self::Args::parse()
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App(name={})", self.name)
    }
}