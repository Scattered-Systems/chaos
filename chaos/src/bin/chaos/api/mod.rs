/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use endpoints::*;

mod endpoints;

pub struct Api {
    pub settings: crate::Settings,
}