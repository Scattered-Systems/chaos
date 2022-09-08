/*
    Appellation: index <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use axum::{extract::Path, routing::get, Json, Router};
use scsys::Timestamp;
use serde_json::json;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Homepage(pub String);

impl Homepage {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn router(&mut self) -> Router {
        let mut router = Router::new();
        router = router.route("/", get(landing));
        router = router.route("/notifications/:id", get(notifications));
        router.clone()
    }
}

impl Default for Homepage {
    fn default() -> Self {
        Self::new("/".to_string())
    }
}

/// Define the landing endpoint
pub async fn landing() -> crate::AxumJson {
    let data = json!({ "timestamp": Timestamp::default() });
    Json(data)
}

/// Implements a notification endpoint
pub async fn notifications(Path(id): Path<usize>) -> crate::AxumJson {
    let data = json!({ "id": id });
    Json(data)
}
