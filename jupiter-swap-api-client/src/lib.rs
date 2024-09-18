use anyhow::{anyhow, Result};
use quote::{QuoteRequest, QuoteResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};

pub mod quote;
mod route_plan_with_metadata;
mod serde_helpers;
pub mod swap;
pub mod transaction_config;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
}

async fn check_is_success(response: Response) -> Result<Response> {
    if !response.status().is_success() {
        return Err(anyhow!(
            "Request status not ok: {}, body: {:?}",
            response.status(),
            response.text().await
        ));
    }
    Ok(response)
}

async fn check_status_code_and_deserialize<T: DeserializeOwned>(response: Response) -> Result<T> {
    check_is_success(response)
        .await?
        .json::<T>()
        .await
        .map_err(Into::into)
}

impl JupiterSwapApiClient {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse> {
        let url = format!("{}/quote", self.base_path);
        let response = Client::new().get(url).query(&quote_request).send().await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap(&self, swap_request: &SwapRequest) -> Result<SwapResponse> {
        let response = Client::new()
            .post(format!("{}/swap", self.base_path))
            .json(swap_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap_instructions(
        &self,
        swap_request: &SwapRequest,
    ) -> Result<SwapInstructionsResponse> {
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
