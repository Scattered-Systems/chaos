/*
    Appellation: storj <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use axum::{extract::Path, routing::{get, post}, Json, Router};
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
        router = router.route("/storj/access/:id", post(authorize));
        router = router.route("/storj/buckets", get(buckets));
        router.clone()
    }
}

pub fn open_project(grant: &str) -> uplink::Project {
    let grant = match uplink::access::Grant::new(grant) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    uplink::Project::open(grant)
}

/// Define the landing endpoint
pub async fn authorize() -> crate::AxumJson {
    let data = json!({ "timestamp": Timestamp::default() });
    Json(data)
}


/// Define the landing endpoint
pub async fn buckets() -> crate::AxumJson {
    let data = json!({ "timestamp": Timestamp::default() });
    Json(data)
}

