/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use blockchain::*;
pub use trees::*;

mod blockchain;
mod trees;

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}