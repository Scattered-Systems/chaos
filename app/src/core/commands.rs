use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Commands {

    #[clap(default_value = "chaos", long, short, value_parser)]
    pub appellation: String
}