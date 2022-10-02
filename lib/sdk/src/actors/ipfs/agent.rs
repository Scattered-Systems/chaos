/*
    Appellation: ipfs <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

use ipfs::{Ipfs, IpfsOptions, IpfsPath, Types, UninitializedIpfs};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum IpfsAgentMode {
    #[default]
    InMemory,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IpfsAgent {
    pub mode: IpfsAgentMode
}

impl IpfsAgent {
    pub fn new(mode: Option<IpfsAgentMode>) -> Self {
        let mode = match mode {
            Some(v) => v,
            None => IpfsAgentMode::default()
        };
        Self { mode }
    }
    pub async fn ipfs(&self, options: Option<IpfsOptions>) -> Ipfs<Types> {
        let options = match options {
            Some(v) => v,
            None => self.in_memory()
        };
        let (ipfs, fut): (Ipfs<Types>, _) = UninitializedIpfs::new(options).start().await.unwrap();
        tokio::spawn(fut);
        ipfs
    }
    pub fn in_memory(&self) -> IpfsOptions {
        IpfsOptions::inmemory_with_generated_keys()
    }
    pub async fn stream_ipfs_path(&self, options: Option<IpfsOptions>, path: &str) -> Vec<String> {
        let ipfs = self.ipfs(options).await;
        let stream = ipfs
            .cat_unixfs(path.parse::<IpfsPath>().unwrap(), None)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });

        tokio::pin!(stream);

        let mut res = Vec::new();

        loop {
            match stream.next().await {
                Some(Ok(bytes)) => {
                    res.push(String::from_utf8(bytes.clone()).ok().unwrap());
                }
                Some(Err(e)) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                None => break,
            }
        }
        res
    }
}

impl Default for IpfsAgent {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_agent() {
        let a = IpfsAgent::default();
        let b = a.clone();
        assert_eq!(a, b)
    }

    #[tokio::test]
    async fn test_ipfs_stream() {
        let path = "/ipfs/QmQPeNsJPyVWPFDVHb77w8G42Fvo15z4bG2X8D2GhfbSXc/readme";
        let agent = IpfsAgent::default();
        println!("{:?}", agent.stream_ipfs_path(None, path).await);
        assert!(true)
    }
}
