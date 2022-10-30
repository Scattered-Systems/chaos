/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use axum::{routing::get, Json, Router};
use scsys::{BoxResult, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BaseRouter;

impl BaseRouter {
    pub fn new() -> Self {
        Self
    }
    pub fn router(&self) -> Router {
        Router::new()
            .route("/", get(landing))
    }
}

pub async fn landing() -> Json<Value> {
    let timestamp = Timestamp::default();
    Json(json!({"timestamp": timestamp}))
}
