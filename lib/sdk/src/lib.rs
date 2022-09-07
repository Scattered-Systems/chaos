/*
    Appellation: chaos-sdk <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use self::{actors::*, cont::*, core::*, data::*};

mod actors;
mod cont;
mod core;
mod data;

pub mod prelude {
    #[cfg(feature = "ipfs")]
    pub use ipfs;
}
