use std::collections::HashMap;

use acme::primitives::{Clock, ObjectId};
use axum;
use serde_json::{json, Value};

pub type Dictionary<T> = HashMap<String, T>;
pub type Container<T> = Dictionary<Vec<T>>;

pub struct Cache {
    pub id: ObjectId,
    pub blocks: Vec<Dictionary<String>>,
    pub timestamp: Clock,
}

impl Cache {
    pub fn new() -> Self {
        let id = ObjectId::new();
        let timestamp: Clock = chrono::Local::now().into();
        let mut tmp: Dictionary<String> = Dictionary::new();
        tmp.insert(String::from("id"), String::from(id.clone()));
        tmp.insert(String::from("timestamp"), String::from(timestamp));
        Self {
            id,
            blocks: vec![tmp],
            timestamp,
        }
    }
}

pub fn create_route() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(base))
}

pub async fn base() -> axum::Json<Value> {
    let mut cache: Dictionary<String> = Dictionary::new();
    let timestamp: bson::DateTime = chrono::Local::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(
        json!(cache)
    )
}

pub async fn store(key: String, value: String) -> axum::Json<Value> {
    let timestamp: bson::DateTime = chrono::Local::now().into();
    let mut cache: Dictionary<String> = Dictionary::new();
    cache.insert(key, value);
    axum::Json(
        json!(cache)
    )
}