/*
    Appellation: commands
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Commands {
    #[clap(default_value = "sample.eth", long, short, value_parser)]
    pub account: String,

    #[clap(default_value = "key", long, short, value_parser)]
    pub key: String,

    #[clap(default_value = "data", long, short, value_parser)]
    pub data: String,
}