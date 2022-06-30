/*
    Appellation: context
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AppContext {
    pub settings: crate::Settings,
}

impl AppContext {
    pub fn new(settings: crate::Settings) -> Self {
        Self { settings }
    }
}

impl std::fmt::Display for AppContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context(configuration={})", self.settings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}