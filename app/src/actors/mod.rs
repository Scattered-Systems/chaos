/*
    Appellation:
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
 */
mod actor;
pub mod apps;
mod context;
pub mod loggers;

pub use actor::*;
pub use apps::*;
pub use context::*;
pub use loggers::*;

pub trait CLI {
    type Commands;

    fn constructor() -> Self::Commands;
}