/*
    Appellation: chaos-sdk <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use self::{actors::*, components::*, core::*, data::*};

mod actors;
mod components;
mod core;
mod data;

pub mod prelude {
    #[cfg(feature = "arweave")]
    pub use arloader;
    #[cfg(feature = "ipfs")]
    pub use ipfs;
}
