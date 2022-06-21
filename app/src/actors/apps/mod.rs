/*
    Appellation: apps
    Context: Module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
 */

pub mod cli;
pub use cli::*;

pub enum AppStates {
    Abort,
    Initialize,
    Compute,
    Create,
    Delete
}