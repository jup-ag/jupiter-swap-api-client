use std::sync::Arc;

use anyhow::anyhow;
use quote::{QuoteRequest, QuoteResponse};
use reqwest::Client;
pub use reqwest::{Request, Response};
pub use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next, Result};
use serde::de::DeserializeOwned;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
pub use task_local_extensions::Extensions;

pub mod quote;
mod route_plan_with_metadata;
mod serde_helpers;
pub mod swap;
pub mod transaction_config;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
    pub client: ClientWithMiddleware,
}

async fn check_is_success(response: Response) -> anyhow::Result<Response> {
    if !response.status().is_success() {
        return Err(anyhow!(
            "Request status not ok: {}, body: {:?}",
            response.status(),
            response.text().await
        ));
    }
    Ok(response)
}

async fn check_status_code_and_deserialize<T: DeserializeOwned>(
    response: Response,
) -> anyhow::Result<T> {
    check_is_success(response)
        .await?
        .json::<T>()
        .await
        .map_err(Into::into)
}

impl JupiterSwapApiClient {
    pub fn new(base_path: String) -> Self {
        let reqwest_client = Client::builder().build().unwrap();
        let client = ClientBuilder::new(reqwest_client).build();
        Self {
            base_path,
            client: client,
        }
    }

    pub fn new_with_middlewares(base_path: String, middlewares: Vec<Arc<dyn Middleware>>) -> Self {
        let reqwest_client = Client::builder().build().unwrap();
        let client = middlewares
            .into_iter()
            .fold(ClientBuilder::new(reqwest_client), |builder, middleware| {
                builder.with_arc(middleware)
            })
            .build();

        Self { base_path, client }
    }

    pub async fn quote(&self, quote_request: &QuoteRequest) -> anyhow::Result<QuoteResponse> {
        let url = format!("{}/quote", self.base_path);
        let response = self.client.get(url).query(&quote_request).send().await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap(&self, swap_request: &SwapRequest) -> anyhow::Result<SwapResponse> {
        let response = self
            .client
            .post(format!("{}/swap", self.base_path))
            .json(swap_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap_instructions(
        &self,
        swap_request: &SwapRequest,
    ) -> anyhow::Result<SwapInstructionsResponse> {
        let response = self
            .client
            .post(format!("{}/swap-instructions", self.base_path))
            .json(swap_request)
            .send()
            .await?;
        check_status_code_and_deserialize::<SwapInstructionsResponseInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_version(&self) -> anyhow::Result<String> {
        let response = self
            .client
            .get(format!("{}/version", self.base_path))
            .send()
            .await?;
        Ok(response.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use async_trait;
    use reqwest::{Client, Request, Response};
    use reqwest_middleware::Next;

    use super::*;

    struct LoggingMiddleware;

    #[async_trait::async_trait]
    impl Middleware for LoggingMiddleware {
        async fn handle(
            &self,
            req: Request,
            extensions: &mut Extensions,
            next: Next<'_>,
        ) -> reqwest_middleware::Result<Response> {
            println!("Request started {:?}", req);
            let res = next.run(req, extensions).await;
            println!("Result: {:?}", res);
            res
        }
    }

    #[tokio::test]
    async fn test_quote() {
        let client = JupiterSwapApiClient::new_with_middlewares(
            "https://api.jup.ag/v6".to_string(),
            vec![Arc::new(LoggingMiddleware {})],
        );

        let version = client.get_version().await.unwrap();
        println!("Version: {}", version);
    }
}
