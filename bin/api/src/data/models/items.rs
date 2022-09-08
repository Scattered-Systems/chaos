/*
    Appellation: items <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::BsonOid;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Items {
    #[serde(rename = "item_id")]
    pub id: BsonOid,
    pub key: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Items {
    pub fn new(label: String, description: Option<String>) -> Self {
        let id = BsonOid::new();
        let key = format!("{}-{}", id.clone(), label.clone());

        Self {
            id,
            key,
            label,
            description,
        }
    }
}
