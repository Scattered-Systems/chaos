/*
    Appellation: app <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::interface::Api;

pub mod endpoints;
pub mod handlers;
mod interface;
pub mod middleware;
