/*
    Appellation: index <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::models::TokenModel;
use axum::{extract::Path, routing::{get, post}, Json, Router};
use scsys::Timestamp;
use serde_json::json;

#[derive(Clone, Debug)]
pub struct Homepage;

impl Homepage {
    pub fn new() -> Self {
        Self
    }
    pub fn router(&self) -> Router {
        let mut router = Router::new();
        router = router.route("/", get(landing));
        router = router.route("/token", post(create_token));
        router = router.route("/notifications/:id", get(notifications));
        router.clone()
    }
}

impl Default for Homepage {
    fn default() -> Self {
        Self::new()
    }
}

/// Define the landing endpoint
pub async fn landing() -> crate::AxumJson {
    let data = json!({ "timestamp": Timestamp::default() });
    Json(data)
}

pub async fn create_token(username: Option<String>) -> crate::AxumJson {
    Json(json!(TokenModel::new(username)))
}

/// Implements a notification endpoint
pub async fn notifications(Path(id): Path<usize>) -> crate::AxumJson {
    let data = json!({ "id": id });
    Json(data)
}
