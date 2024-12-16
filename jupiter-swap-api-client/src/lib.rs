use std::sync::Arc;

pub use http::Extensions;
use quote::{InternalQuoteRequest, QuoteRequest, QuoteResponse};
use reqwest::Client;
pub use reqwest::{Request, Response};
pub use reqwest_middleware::{
    ClientBuilder, ClientWithMiddleware, Middleware, Next, Result as MiddlewareResult,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use thiserror::Error;
pub mod quote;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod swap;
pub mod transaction_config;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
    pub client: ClientWithMiddleware,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Request failed with status {status}: {body}")]
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] reqwest::Error),
}

async fn check_is_success(response: Response) -> Result<Response, ClientError> {
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(ClientError::RequestFailed { status, body });
    }
    Ok(response)
}

async fn check_status_code_and_deserialize<T: DeserializeOwned>(
    response: Response,
) -> Result<T, ClientError> {
    let response = check_is_success(response).await?;
    response
        .json::<T>()
        .await
        .map_err(ClientError::DeserializationError)
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
    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse, ClientError> {
        let url = format!("{}/quote", self.base_path);
        let extra_args = quote_request.quote_args.clone();
        let internal_quote_request = InternalQuoteRequest::from(quote_request.clone());
        let response = Client::new()
            .get(url)
            .query(&internal_quote_request)
            .query(&extra_args)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap(
        &self,
        swap_request: &SwapRequest,
        extra_args: Option<HashMap<String, String>>,
    ) -> Result<SwapResponse, ClientError> {
        let response = Client::new()
            .post(format!("{}/swap", self.base_path))
            .query(&extra_args)
            .json(swap_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap_instructions(
        &self,
        swap_request: &SwapRequest,
    ) -> Result<SwapInstructionsResponse, ClientError> {
        let response = Client::new()
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
    use reqwest::{Request, Response};
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
