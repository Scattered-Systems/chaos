/*
   Appellation: interface <module>
   Creator: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
        ... Summary ...
*/
use clap::Parser;
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Parser, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "Welcome to the Chaos")]
pub struct Cli {
    #[clap(arg_enum)]
    pub args: Option<AppArgs>,
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn new() -> Self {
        println!("Gathering inputs...");
        Self::parse()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Clone,
    Debug,
    Hash,
    EnumString,
    EnumVariantNames,
    PartialEq,
    clap::ArgEnum,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum AppArgs {
    Account,
    Update,
}

#[derive(
    Clone,
    Debug,
    Hash,
    EnumString,
    EnumVariantNames,
    PartialEq,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Commands {
    Ipfs {
        #[clap(long, value_parser)]
        cid: String,
    },
    Storj {
        #[clap(long, short, value_parser)]
        bucket: Option<String>,
    },
    Null,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Null
    }
}
