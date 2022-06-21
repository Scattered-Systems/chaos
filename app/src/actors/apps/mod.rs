/*
    Appellation: apps
    Context: Module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
 */

pub mod application;
pub use application::*;

pub enum AppStates {
    Abort,
    Initialize,
    Compute,
    Create,
    Delete
}