/*
    Appellation: crud <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use axum::{extract::Path, routing::get};
use serde_json::json;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CrudRouter {
    pub path: String,
}

impl CrudRouter {
    pub fn new(path: String) -> Self {
        Self {
            path: path.to_string(),
        }
    }
    pub fn from<T: std::string::ToString>(path: T) -> Self {
        Self::new(path.to_string())
    }
    pub fn endpoint(&self, prefix: &str) -> String {
        format!("{}{}", self.path.clone(), prefix)
    }
    pub fn router(&self) -> axum::Router {
        let mut router = axum::Router::new();
        router = router.route("/crud/:uid", get(crud_base));
        router
    }
}

impl Default for CrudRouter {
    fn default() -> Self {
        Self::from("/crud")
    }
}

pub async fn crud_base(Path(uid): Path<String>) -> crate::AxumJson {
    axum::Json(json!({ "id": uid }))
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
struct FlashData {
    kind: String,
    message: String,
}
