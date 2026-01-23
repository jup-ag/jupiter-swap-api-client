use std::collections::HashMap;

use quote::{InternalQuoteRequest, QuoteRequest, QuoteResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use thiserror::Error;
use tracing::debug;

pub mod quote;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod swap;
pub mod transaction_config;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
    pub api_key: String,
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
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] serde_json::Error),
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

    // Get the raw response text for logging
    let response_text = response.text().await?;

    // Log the raw response at debug level
    debug!(
        response_length = response_text.len(),
        "Jupiter API raw response: {}", 
        response_text
    );

    // Attempt to deserialize from the text
    serde_json::from_str::<T>(&response_text).map_err(|e| {
        debug!("JSON deserialization error: {}", e);
        ClientError::JsonParseError(e)
    })
}

impl JupiterSwapApiClient {
    pub fn new(base_path: String, api_key: String) -> Self {
        Self { base_path, api_key }
    }

    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse, ClientError> {
        let url = format!("{}/quote", self.base_path);
        let extra_args = quote_request.quote_args.clone();
        let internal_quote_request = InternalQuoteRequest::from(quote_request.clone());
        let response = Client::new()
            .get(url)
            .query(&internal_quote_request)
            .query(&extra_args)
            .header("x-api-key", &self.api_key)
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
            .header("x-api-key", &self.api_key)
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
            .header("x-api-key", &self.api_key)
            .send()
            .await?;
        check_status_code_and_deserialize::<SwapInstructionsResponseInternal>(response)
            .await
            .map(Into::into)
    }
}
