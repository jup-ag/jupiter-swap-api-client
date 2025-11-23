# 🚀 jupiter-swap-api-client: Seamless Solana Swaps in Rust

## Introduction

The `jupiter-swap-api-client` is an unofficial **Rust client library** designed for easy, type-safe integration with the **Jupiter Swap API (v6)**. It allows Rust applications to effortlessly fetch the best quotes and execute decentralized swaps on the Solana blockchain.

---

## ✨ Key Features

* **Type Safety:** Uses idiomatic Rust structs and enums for API requests and responses.
* **Asynchronous:** Built on `tokio` for non-blocking network operations.
* **Simple Integration:** Abstracts complex HTTP request logic for fetching quotes and transactions.
* **Flexible Deployment:** Supports both the official Jupiter API and self-hosted instances.

---

## 🛠 Getting Started

To use the `jupiter-swap-api-client` crate in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
# Note: Since this package is likely sourced directly from the GitHub repo,
# we specify the git dependency. Check Crates.io for a released version.
jupiter-swap-api-client = { git = "[https://github.com/jup-ag/jupiter-swap-api-client.git](https://github.com/jup-ag/jupiter-swap-api-client.git)" }
# Required for running async main function
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
# Required for Solana types like Pubkey and the pubkey! macro (ensure this feature is enabled)
solana-sdk = { version = "1.18", features = ["full"] }
⚡ Examples
This example demonstrates fetching a quote for swapping 1 USDC for Native SOL and then requesting the raw transaction data for the swap.
use jupiter_swap_api_client::{
    quote::QuoteRequest, 
    swap::SwapRequest, 
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::{Pubkey, pubkey}; // Import the pubkey! macro helper

// Define standard mints and a test wallet
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm");

// Recommended base URL for Jupiter V6 Quote API
const JUPITER_API_URL: &str = "[https://quote-api.jup.ag/v6](https://quote-api.jup.ag/v6)";

#[tokio::main]
async fn main() {
    let jupiter_swap_api_client = JupiterSwapApiClient::new(JUPITER_API_URL);

    // Request parameters: 1,000,000 lamports (1 USDC), 50 basis points (0.5%) slippage
    let quote_request = QuoteRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: NATIVE_MINT,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // 1. Fetch the best route quote (GET /quote)
    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.expect("Failed to fetch quote");
    println!("--- Quote Response ---");
    println!("{quote_response:#?}");

    // 2. Request the raw, serialized transaction for the swap (POST /swap)
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: TEST_WALLET,
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await
        .expect("Failed to fetch swap transaction");

    println!("\n--- Swap Transaction ---");
    println!("Raw serialized transaction length: {}", swap_response.swap_transaction.len());

    // NOTE: swap_response.swap_transaction must be deserialized, signed by the user, and sent to the Solana RPC.

    // 3. Alternatively, request the serialized instructions array (POST /swap-instructions)
    let swap_instructions = jupiter_swap_api_client
        .swap_instructions(&SwapRequest {
            user_public_key: TEST_WALLET,
            quote_response,
            config: TransactionConfig::default(),
        })
        .await
        .expect("Failed to fetch swap instructions");
        
    println!("\n--- Swap Instructions ---");
    println!("{swap_instructions:#?}");
}
⚙️ Using Self-hosted or Hosted APIs
The client defaults to using the official Jupiter API. To use a self-hosted or paid hosted API instance, you can override the base URL via an environment variable.

Environment Variable Configuration
Set the API_BASE_URL environment variable before running your application to point to your custom API endpoint:

Bash

# Example for a self-hosted API instance
export API_BASE_URL="[https://hosted.api](https://hosted.api)"
# Then run your Rust application
cargo run
Paid Hosted APIs
For documentation on self-hosting or utilizing Jupiter's paid hosted APIs, refer to the official documentation.

📚 Additional Resources
Jupiter Swap API Documentation: https://station.jup.ag/docs/v6/swap-api — Dive deeper into the API endpoint details.


Shutterstock
jup.ag Website: https://jup.ag/ — Explore the official website and interface.
