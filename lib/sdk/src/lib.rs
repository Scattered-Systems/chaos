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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
