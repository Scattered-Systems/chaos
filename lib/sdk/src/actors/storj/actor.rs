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
    passphrase: String,
}

impl StorjActor {
    pub fn new(access: String, passphrase: String) -> Self {
        Self { access, passphrase }
    }
    pub fn from(data: &Self) -> Self {
        Self::new(data.access.clone(), data.passphrase.clone())
    }
    pub fn from_env() -> Self {
        let access = match std::env::var_os("STORJ_ACCESS_GRANT") {
            Some(v) => v.into_string().ok().unwrap(),
            None => String::new(),
        };
        let passphrase = match std::env::var_os("STORJ_PASSPHRASE") {
            Some(v) => v.into_string().ok().unwrap(),
            None => String::new(),
        };
        Self::new(access, passphrase)
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
