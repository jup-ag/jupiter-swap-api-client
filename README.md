# jup-swap-api-client

## Introduction

The `jup-swap-api-client` is a Rust client library designed to simplify the integration of the Jupiter Swap API, enabling seamless swaps on the Solana blockchain.

## Getting Started

To use the `jup-swap-api-client` crate in your Rust project, follow these simple steps:

Add the crate to your `Cargo.toml`:

    ```toml
    [dependencies]
    jupiter-swap-api-client = { git = "https://github.com/jup-ag/jupiter-swap-api-client.git", package = "jupiter-swap-api-client"}
    ```

## Examples

Here's a simplified example of how to use the `jup-swap-api-client` in your Rust application:

```rust
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm");

#[tokio::main]
async fn main() {
    let jupiter_swap_api_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6");

    let quote_request = QuoteRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: NATIVE_MINT,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // GET /quote
    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
    println!("{quote_response:#?}");

    // POST /swap
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: TEST_WALLET,
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();

    println!("Raw tx len: {}", swap_response.swap_transaction.len());

    // Perform further actions as needed...

    // POST /swap-instructions
    let swap_instructions = jupiter_swap_api_client
        .swap_instructions(&SwapRequest {
            user_public_key: TEST_WALLET,
            quote_response,
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();
    println!("{swap_instructions:#?}");
}

```
For the full example, please refer to the [examples](./example/) directory in this repository.

### Using Self-hosted APIs

You can set custom URLs via environment variables for any self-hosted Jupiter APIs. Like the [V6 Swap API](https://station.jup.ag/docs/apis/self-hosted) or the [paid hosted APIs](#paid-hosted-apis). Here are the ENV vars:

```
API_BASE_URL=https://hosted.api
```

### Paid Hosted APIs

You can also check out some of the [paid hosted APIs](https://station.jup.ag/docs/apis/self-hosted#paid-hosted-apis).

## Additional Resources

- [Jupiter Swap API Documentation](https://station.jup.ag/docs/v6/swap-api): Learn more about the Jupiter Swap API and its capabilities.
- [jup.ag Website](https://jup.ag/): Explore the official website for additional information and resources.
