/*
    Appellation: gateway <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use s3::{creds::Credentials, error::S3Error, Bucket, Region};
use scsys::prelude::Server;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GatewaySettings {

    pub server: Server
}

pub struct GatewayContext {
    pub settings: GatewaySettings
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gateway {
    creds: Credentials,
    region: Region
}

impl Gateway {
    pub fn new(creds: Credentials, region: Region) -> Self {
        Self { creds, region }
    }
    pub fn bucket(&self, name: &str) -> Result<Bucket, S3Error> {
        Bucket::new(name, self.region.clone(), self.creds.clone())
    }
}