use std::collections::HashMap;

use quote::{InternalQuoteRequest, QuoteRequest, QuoteResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use thiserror::Error;

use crate::build::{BuildRequest, InternalBuildRequest};

pub mod build;
pub mod quote;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod swap;
pub mod transaction_config;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Request failed with status {status}: {body}")]
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("Failed to read response body: {0}")]
    BodyReadError(#[from] reqwest::Error),
    #[error("Failed to deserialize response at path `{path}`: {source}")]
    DeserializationError {
        path: String,
        source: serde_json::Error,
    },
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
    let text = response.text().await?;
    let jd = &mut serde_json::Deserializer::from_str(&text);
    serde_path_to_error::deserialize(jd).map_err(|e| ClientError::DeserializationError {
        path: e.path().to_string(),
        source: e.into_inner(),
    })
}

impl JupiterSwapApiClient {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    /// Fetches a quote and builds raw swap instructions in a single call using the `/build` endpoint.
    ///
    /// This is the V2 equivalent of calling `quote` + `swap_instructions` separately (V1/Metis required
    /// two calls: `GET /swap/v1/quote` then `POST /swap/v1/swap-instructions`).
    ///
    /// Routing is handled by **Metis**, Jupiter's onchain routing engine, which finds the optimal
    /// swap path across Solana DEXes. Unlike the assembled-transaction endpoint, this returns raw
    /// instructions, giving you full control to:
    /// - Add custom instructions before/after the swap
    /// - Integrate via CPI
    /// - Modify any part of the transaction
    ///
    /// Once built and signed, submit the transaction via your own RPC or use `/submit` to land it
    /// through Jupiter's transaction infrastructure with SOL tips.
    ///
    /// # V2 Changes
    /// - **Base URL**: `https://api.jup.ag/swap/v2` (previously `https://api.jup.ag/swap/v1`)
    /// - **Single call**: `GET /swap/v2/build` (previously two calls: quote + swap-instructions)
    /// - **`routePlan`**: fees expressed in **bps** (previously `percent` in V1)
    /// - **Instruction format**: V2 format (incompatible with V1)
    ///
    /// # Requires
    /// A V2 base URL, e.g. `https://api.jup.ag/swap/v2`
    pub async fn build(&self, build_request: &BuildRequest) -> Result<SwapInstructionsResponse, ClientError> {
      let url = format!("{}/build", self.base_path);
      let internal_quote_request = InternalBuildRequest::from(build_request.clone());
      let response = Client::new()
          .get(url)
          .query(&internal_quote_request)
          .send()
          .await?;
      check_status_code_and_deserialize::<SwapInstructionsResponseInternal>(response)
          .await
          .map(Into::into)
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
}
