use std::env;

use autobahn_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use solana_sdk::{pubkey::Pubkey, signature::NullSigner};

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet

#[tokio::main]
async fn main() {
    let api_base_url = env::var("API_BASE_URL").unwrap_or("http://localhost:8888".into());
    println!("Using base url: {}", api_base_url);

    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

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
        .swap(
            &SwapRequest {
                user_public_key: "2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm".into(),
                quote_response: quote_response.clone(),
                wrap_and_unwrap_sol: false,
                auto_create_out_ata: false,
                use_shared_accounts: false,
                fee_account: None,
                compute_unit_price_micro_lamports: None,
                as_legacy_transaction: false,
                use_token_ledger: false,
                destination_token_account: None,
            },
            None,
        )
        .await
        .unwrap_or_else(|e| {
            eprintln!("Error during swap. Raw api response: {}", e);
            std::process::exit(1);
        });

    println!("Raw tx len: {}", swap_response.swap_transaction.len());

    let versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction).unwrap();

    // Replace with a keypair or other struct implementing signer
    let null_signer = NullSigner::new(&TEST_WALLET);
    //let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&null_signer]).unwrap();

    // send with rpc client...
    //let rpc_client = RpcClient::new("https://solana-rpc.publicnode.com".into());

    // This will fail with "Transaction signature verification failure" as we did not really sign
    /* let error = rpc_client
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await
        .unwrap_err();
    println!("{error}"); */

    // POST /swap-instructions
    let swap_instructions = jupiter_swap_api_client
        .swap_instructions(&SwapRequest {
            user_public_key: "2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm".into(),
            quote_response: quote_response.clone(),
            wrap_and_unwrap_sol: false,
            auto_create_out_ata: false,
            use_shared_accounts: false,
            fee_account: None,
            compute_unit_price_micro_lamports: None,
            as_legacy_transaction: false,
            use_token_ledger: false,
            destination_token_account: None,
        })
        .await
        .unwrap();
    println!("swap_instructions: {swap_instructions:#?}");
}
