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

impl TokenModel {}
