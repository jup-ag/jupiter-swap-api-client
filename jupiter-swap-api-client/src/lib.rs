use std::collections::HashMap;
use std::sync::Arc;

use quote::{InternalQuoteRequest, QuoteRequest, QuoteResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use thiserror::Error;

pub mod quote;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod swap;
pub mod transaction_config;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Request failed with status {status}: {body}")]
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
}

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
    // Optimization: Shared HTTP client for connection pooling
    client: Client,
}

impl JupiterSwapApiClient {
    /// Creates a new Jupiter API client with connection pooling.
    pub fn new(base_path: String) -> Self {
        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { base_path, client }
    }

    /// Internal helper to validate response and deserialize JSON.
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, ClientError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
            return Err(ClientError::RequestFailed { status, body });
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ClientError::DeserializationError(e.to_string()))
    }

    /// Gets a quote for a swap.
    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse, ClientError> {
        let url = format!("{}/quote", self.base_path);
        let extra_args = &quote_request.quote_args;
        let internal_quote_request = InternalQuoteRequest::from(quote_request.clone());

        let response = self.client
            .get(url)
            .query(&internal_quote_request)
            .query(extra_args)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Performs a swap.
    pub async fn swap(
        &self,
        swap_request: &SwapRequest,
        extra_args: Option<&HashMap<String, String>>,
    ) -> Result<SwapResponse, ClientError> {
        let url = format!("{}/swap", self.base_path);
        
        let response = self.client
            .post(url)
            .query(&extra_args)
            .json(swap_request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Gets instructions for a swap without executing it.
    pub async fn swap_instructions(
        &self,
        swap_request: &SwapRequest,
    ) -> Result<SwapInstructionsResponse, ClientError> {
        let url = format!("{}/swap-instructions", self.base_path);

        let response = self.client
            .post(url)
            .json(swap_request)
            .send()
            .await?;

        let internal_res: SwapInstructionsResponseInternal = self.handle_response(response).await?;
        Ok(internal_res.into())
    }
}
