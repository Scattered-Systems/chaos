/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use interfaces::*;

mod interfaces;

pub trait Context {
    type Actor;
    type Config;
    type Data;

    fn constructor(&self, settings: Self::Config) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}