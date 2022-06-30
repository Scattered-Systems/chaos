/*
    Appellation: interfaces
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

pub trait Conduit {
    type Actor;
    type Client;
    type Config;
    type Data;

    fn actor(&self, actions: Self::Data) -> Self::Actor;
    fn client(&self) -> Result<Self::Client, Box<dyn std::error::Error + Send + Sync + 'static>>;
    fn configure(&self, configuration: Self::Config) -> Result<Self::Config, config::ConfigError>;
    fn determine(&self, data: Self::Data) -> Self::Data;
}

pub trait CommandLineInterface {
    type Args;

    fn client(&self) -> Self::Args;
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}