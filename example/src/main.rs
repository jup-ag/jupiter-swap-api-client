use std::env;

use jupiter_swap_api::{
    quote::{QuoteRequest, QuoteResponse},
    swap::{SwapRequest, SwapResponse},
    transaction_config::TransactionConfig,
};
use reqwest::Client;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use tokio;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet

#[tokio::main]
async fn main() {
    let quote_api_base_url =
        env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v5".into());
    println!("Using base url: {}", quote_api_base_url);

    let quote_request = QuoteRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: NATIVE_MINT,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };
    let query = serde_qs::to_string(&quote_request).unwrap();
    println!("The quote query is {}", query);

    let client = Client::new();
    let quote_response = client
        .get(format!("{quote_api_base_url}/quote?{query}"))
        .send()
        .await
        .unwrap()
        .json::<QuoteResponse>()
        .await
        .unwrap();
    println!("{quote_response:#?}");

    let swap_response = client
        .post(format!("{quote_api_base_url}/swap"))
        .json(&SwapRequest {
            user_public_key: TEST_WALLET,
            quote_response,
            config: TransactionConfig::default(),
        })
        .send()
        .await
        .unwrap()
        .json::<SwapResponse>()
        .await
        .unwrap();
    println!("Raw tx len: {}", swap_response.swap_transaction.len());
}
