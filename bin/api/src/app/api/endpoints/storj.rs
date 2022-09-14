/*
    Appellation: storj <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use axum::{extract::Path, routing::get, Json, Router};
use scsys::Timestamp;
use serde_json::json;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StorjRouter(pub String);

impl StorjRouter {
    pub fn new() -> Self {
        Self("/storj".to_string())
    }
    pub fn endpoint(&self, path: Option<&str>) -> String {
        match path {
            Some(v) => format!("{}/{}", self.0, v),
            None => self.0.clone()
        }
    }
    pub fn router(&mut self) -> Router {
        let mut router = Router::new();
        router = router.route(self.endpoint(Some("/buckets")).as_str(), get(buckets));
        router.clone()
    }
}

/// Define the landing endpoint
pub async fn buckets() -> crate::AxumJson {
    let data = json!({ "timestamp": Timestamp::default() });
    Json(data)
}

