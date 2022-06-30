/*
    Appellation: actors
    Context: module
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use actor::*;
pub use agents::*;

mod agents;

mod actor {}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}