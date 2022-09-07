/*
    Appellation: core <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{primitives::*, utils::*};

mod primitives;

mod utils {
    use crate::{
        prelude::ipfs::{IpfsOptions, IpfsPath},
        Ipfs,
    };
    use std::process::exit;
    use tokio_stream::StreamExt;

    pub async fn ipfs_test_client() -> Ipfs {
        let opts = IpfsOptions::inmemory_with_generated_keys();
        let (ipfs, fut): (Ipfs, _) = ipfs::UninitializedIpfs::new(opts).start().await.unwrap();
        tokio::task::spawn(fut);

        ipfs
    }

    pub async fn stream_ipfs_path(options: IpfsOptions, path: &str) -> Vec<String> {
        let (ipfs, fut): (Ipfs, _) = ipfs::UninitializedIpfs::new(options).start().await.unwrap();
        // Spawn the background task
        tokio::task::spawn(fut);
        // Restore the default bootstrappers to enable content discovery
        ipfs.restore_bootstrappers().await.unwrap();
        let stream = ipfs
            .cat_unixfs(path.parse::<IpfsPath>().unwrap(), None)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                exit(1);
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
                    exit(1);
                }
                None => break,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::stream_ipfs_path;

    #[tokio::test]
    async fn test_ipfs_stream() {
        let path = "/ipfs/QmQPeNsJPyVWPFDVHb77w8G42Fvo15z4bG2X8D2GhfbSXc/readme";
        let a = stream_ipfs_path(ipfs::IpfsOptions::inmemory_with_generated_keys(), path).await;
        let b = a.clone();
        assert_eq!(a, b)
    }
}
