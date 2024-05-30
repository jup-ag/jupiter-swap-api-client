use std::time::Duration;
use anyhow::{anyhow, Result};
use quote::{QuoteRequest, QuoteResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use tokio::time::sleep;

pub mod quote;
mod route_plan_with_metadata;
mod serde_helpers;
pub mod swap;
pub mod transaction_config;


const MAX_RETRIES: usize = 3;
const RETRY_DELAY: Duration = Duration::from_secs(1);

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
        let query = serde_qs::to_string(&quote_request)?;
        let response = Client::new()
            .get(format!("{}/quote?{query}", self.base_path))
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn retry_quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse> {
        let query = serde_qs::to_string(&quote_request)?;

        for attempt in 0..MAX_RETRIES {
            let response = Client::new()
                .get(format!("{}/quote?{}", self.base_path, query))
                .send()
                .await;

            match response {
                Ok(res) => match check_status_code_and_deserialize(res).await {
                    Ok(quote_response) => return Ok(quote_response),
                    Err(err) => {
                        if attempt == MAX_RETRIES - 1 {
                            return Err(err);
                        }
                    }
                },
                Err(err) => {
                    if attempt == MAX_RETRIES - 1 {
                        return Err(err.into());
                    }
                }
            }

            sleep(RETRY_DELAY).await;
        }

        Err(anyhow!("Max retries reached"))
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
