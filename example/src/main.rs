use std::env;
// Use a generic error type for simplified error propagation in main.
use anyhow::Result;

use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use solana_sdk::{pubkey::Pubkey, signature::NullSigner};

// --- CONSTANTS: MINT ADDRESSES AND WALLET ---

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

// Test wallet address used for simulating the swap transaction.
pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm");

#[tokio::main]
// Use anyhow::Result for ergonomic error handling throughout the asynchronous main function.
async fn main() -> Result<()> {
    // Determine the Jupiter API base URL, falling back to the standard endpoint.
    let api_base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "https://quote-api.jup.ag/v6".into());
    println!("Using Jupiter base url: {}", api_base_url);

    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

    // --- 1. GET /quote ---
    
    // Request a quote for swapping 1,000,000 USDC (6 decimals) into SOL (native mint).
    let quote_request = QuoteRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: NATIVE_MINT,
        // Restrict the route search to specific DEXes for potential latency reduction.
        dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
        slippage_bps: 50, // 0.5% slippage tolerance
        ..QuoteRequest::default()
    };

    let quote_response = jupiter_swap_api_client.quote(&quote_request).await?;
    println!("Quote Response: {quote_response:#?}");

    // --- 2. POST /swap ---

    // Request the serialized swap transaction from the API.
    let swap_request = SwapRequest {
        user_public_key: TEST_WALLET,
        quote_response: quote_response.clone(),
        config: TransactionConfig::default(),
    };

    let swap_response = jupiter_swap_api_client.swap(&swap_request, None).await?;
    println!("Raw serialized transaction length: {}", swap_response.swap_transaction.len());

    // Deserialize the raw transaction bytes into a Solana VersionedTransaction struct.
    let versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction)?;

    // --- 3. SIMULATE TRANSACTION SENDING ---
    
    // NOTE: This part demonstrates the signing and sending flow but will FAIL
    // on the network because the transaction is signed with a NullSigner.
    
    // Create a NullSigner using the test wallet key (does not hold the actual private key).
    let null_signer = NullSigner::new(&TEST_WALLET);
    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&null_signer])?;

    // Determine the RPC client URL, prioritizing environment variable for flexibility.
    let rpc_url = env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".into());
    let rpc_client = RpcClient::new(rpc_url);

    // Attempt to send the transaction (expected to fail due to bad signature).
    match rpc_client
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await
    {
        Ok(_) => println!("Unexpected success! (Check why the NullSigner worked)"),
        Err(error) => println!("Transaction failed as expected (Signature verification failed): {error}"),
    }

    // --- 4. POST /swap-instructions ---
    
    // Alternatively, request only the instruction details (not the serialized transaction).
    let swap_instructions = jupiter_swap_api_client
        .swap_instructions(&swap_request)
        .await?;
        
    println!("\nSwap Instructions Details: {swap_instructions:?}");
    
    Ok(())
}
