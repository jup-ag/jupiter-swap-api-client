# jup-swap-api-client

## Introduction

The `jup-swap-api-client` is a Rust client library designed to simplify the integration of the Jupiter Swap API, enabling seamless swaps on the Solana blockchain.

## Migration to New Jupiter API

Jupiter is migrating from the lite API to the new production API. **The lite API will be deprecated on December 31, 2025.**

### Key Changes:
- **New Base URL**: `https://api.jup.ag` (previously `https://quote-api.jup.ag` or `https://lite-api.jup.ag`)
- **API Key Required**: The new production API requires authentication via the `x-api-key` header
- **Rate Limits**: Free tier provides 60 requests per minute

### Getting Your API Key:
1. Visit [portal.jup.ag](https://portal.jup.ag)
2. Connect via email
3. Generate your API key

For more details, see the [official migration guide](https://dev.jup.ag/portal/migrate-from-lite-api).

## Getting Started

To use the `jup-swap-api-client` crate in your Rust project, follow these simple steps:

Add the crate to your `Cargo.toml`:

    ```toml
    [dependencies]
    jupiter-swap-api-client = { git = "https://github.com/jup-ag/jupiter-swap-api-client.git", package = "jupiter-swap-api-client"}
    ```

## Examples

### Using the New Production API (Recommended)

Here's how to use the new production API with authentication:

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
    // Using the new production API with API key (recommended)
    let api_key = "your-api-key-here".to_string();
    let jupiter_swap_api_client = JupiterSwapApiClient::with_api_key(
        "https://api.jup.ag/swap/v1".to_string(),
        api_key
    );

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

### Legacy API Support (Deprecated)

For backward compatibility, the client can still be used without an API key:

```rust
// Without API key (for legacy or self-hosted APIs)
let jupiter_swap_api_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".to_string());
```

**Note**: The lite/quote API will be deprecated on December 31, 2025. Please migrate to the new production API.

### Using Environment Variables

You can configure the API via environment variables:

```bash
# Set the API base URL (defaults to https://api.jup.ag/v6)
export API_BASE_URL=https://api.jup.ag/v6

# Set your Jupiter API key for authentication
export JUPITER_API_KEY=your-api-key-here
```

### Using Self-hosted APIs

You can set custom URLs via environment variables for any self-hosted Jupiter APIs. Like the [V6 Swap API](https://station.jup.ag/docs/apis/self-hosted) or the [paid hosted APIs](#paid-hosted-apis). Here are the ENV vars:

```bash
API_BASE_URL=https://hosted.api
JUPITER_API_KEY=your-api-key  # if required by your hosted API
```

### Paid Hosted APIs

You can also check out some of the [paid hosted APIs](https://station.jup.ag/docs/apis/self-hosted#paid-hosted-apis).

## Additional Resources

- [Jupiter Swap API Documentation](https://station.jup.ag/docs/v6/swap-api): Learn more about the Jupiter Swap API and its capabilities.
- [jup.ag Website](https://jup.ag/): Explore the official website for additional information and resources.
