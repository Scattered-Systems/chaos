/*
    Appellation: options
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
/*
    Appellation: commands
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

#[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
pub struct CliOptions {
    #[clap(default_value = "sample.eth", long, short, value_parser)]
    pub account: String,

    #[clap(default_value = "key", long, short, value_parser)]
    pub key: String,

    #[clap(default_value = "data", long, short, value_parser)]
    pub data: String,
}