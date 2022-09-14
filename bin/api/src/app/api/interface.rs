/*
   Appellation: interface <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::{app::api::endpoints, Context};
use http::header::{HeaderName, AUTHORIZATION};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Api {
    pub address: SocketAddr,
    pub context: Context,
}

impl Api {
    pub fn new(context: Context) -> Self {
        Self {
            address: context.clone().settings.server.address(),
            context,
        }
    }
    pub async fn client(&self) -> scsys::BoxResult<axum::Router> {
        let client = axum::Router::new()
            .merge(endpoints::Homepage::default().router())
            .merge(endpoints::CrudRouter::default().router())
            .merge(endpoints::StorjRouter::default().router())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(axum::Extension(self.context.clone()));
        Ok(client)
    }
    /// Implements a graceful shutdown when users press CTRL + C
    pub async fn shutdown(&self) {
        tokio::signal::ctrl_c()
            .await
            .expect("Expect shutdown signal handler");
        println!("signal shutdown");
    }
    /// Quickly run the api
    pub async fn run(&self) -> scsys::BoxResult {
        let client = self.client().await.expect("Client error...").clone();
        let server = axum::Server::bind(&self.address.clone())
            .serve(client.into_make_service())
            .with_graceful_shutdown(self.shutdown())
            .await
            .expect("Server error");
        Ok(server)
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the application locally at http://localhost:{}",
            self.context.settings.server.port
        )
    }
}
