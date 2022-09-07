/*
    Appellation: actor <storj>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{StorjBucket, StorjGrant, StorjProject};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StorjActor {
    access: String,
}

impl StorjActor {
    pub fn new(access: String) -> Self {
        Self { access }
    }
    pub fn grant(&self) -> StorjGrant {
        match StorjGrant::new(self.access.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }
    pub fn open_project(&self) -> StorjProject {
        uplink::Project::open(self.grant())
    }
    pub fn buckets(&self) -> Vec<StorjBucket> {
        self.open_project()
            .list_buckets(None)
            .map(|i| i.expect(""))
            .collect::<Vec<_>>()
    }
    pub fn bucket_names(&self) -> Vec<String> {
        self.buckets()
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<String>>()
    }
}
