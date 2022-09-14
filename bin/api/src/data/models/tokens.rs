/*
    Appellation: tokens <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TokenModel {
    access_token: String,
    token_type: String,
    username: Option<String>,
}

impl TokenModel {
    pub fn new(username: Option<String>) -> Self {
        Self { access_token: String::new(), token_type: String::new(), username }
    }

}

impl Default for TokenModel {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::TokenModel;

    #[test]
    fn test_default_token() {
        let a = TokenModel::default();
        assert_eq!(a.username, None)
    }
}