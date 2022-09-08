/*
   Appellation: core <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{context::Context, primitives::*, settings::Settings, utils::*};

mod context;
mod settings;

mod primitives {
    /// Type alias for [axum::Json] with a default set equal to [serde_json::Value]
    pub type AxumJson<T = serde_json::Value> = axum::Json<T>;
}

mod utils {}
