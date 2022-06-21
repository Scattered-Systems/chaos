/*
    Appellation: disarray
    Context: library
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
#[doc(inline)]
#[cfg(feature = "default")]
pub use disarray_core::*;

mod actors;
mod controllers;
mod core;
mod data;

pub use crate::{actors::*, controllers::*, core::*, data::*};