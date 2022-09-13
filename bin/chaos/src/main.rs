
pub use self::core::*;


mod core;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Welcome to the Chaos!");

    let chaos = Chaos::new();
    let _storj = chaos.context.settings.provider.expect("").clone();

    println!("{:?}", _storj.clone());
    let _exchange = clients::Exchange::new();
    println!("{:?}", _exchange.currencies().await);

    Ok(())
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Chaos {
    pub context: Context
}

impl Chaos {
    pub fn new() -> Self {
        let settings = Settings::default();
        let context = Context::new(settings);

        Self { context }
    }

    pub fn get_context(&self) -> Context {
        self.context.clone()
    }

    pub fn get_settings(&self) -> Settings {
        self.context.settings.clone()
    }
}


pub mod clients {
    use reqwest::Method;

    #[derive(Clone, Debug)]
    pub struct Exchange {
        pub endpoint: String,
        pub headers: reqwest::header::HeaderMap
    }

    impl Exchange {
        pub fn new() -> Self {
            let endpoint = "https://api.exchange.coinbase.com".to_string();
            let mut headers = reqwest::header::HeaderMap::new();
            headers.append("Accept", "application/json".parse().unwrap());
            headers.append("User-Agent", acme::ME_USER_AGENT.parse().unwrap());
            Self { endpoint, headers }
        }
        pub fn client(&self) -> reqwest::Client {
            reqwest::Client::new()
        }
        pub fn request(&self, method: Method, path: Option<&str>) -> reqwest::RequestBuilder {
            let url = match path {
                Some(v) => format!("{}{}", self.endpoint, v),
                None => self.endpoint.clone()
            };

            self.client().request(method, url)
        }

        pub async fn currencies(&self) -> serde_json::Value {
            self.client()
                .get(format!("{}/currencies", self.endpoint))
                .headers(self.headers.clone())
                .send()
                .await.expect("Response")
                .json()
                .await.expect("Extract")

        }
    }

}