/*
    Appellation: chaos-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "ipfs")]
pub use chaos_ipfs as ipfs;

pub mod prelude {
    #[cfg(feature = "ipfs")]
    pub use super::ipfs::*;
}
