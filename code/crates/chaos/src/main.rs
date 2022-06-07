mod apps;
mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let rpc = apps::api::interface::Interface::new().await;
    Ok(())
}

pub mod networking {
    use libp2p;

    trait Client {
        type State;
        fn get(self) -> String;
    }

    pub struct Node {
        pub address: String,
        pub endpoint: String,
    }

    impl Client for Node {
        type State = ();
        fn get(self) -> String {
            return self.address;
        }
    }
}

