// This example demonstrates how to connect to the Polygon blockchain and retrieve basic information.
// It initializes the blockchain provider and fetches the chain ID, current block number, and gas price.

use core::blockchain;
use ethers::{prelude::*, utils::format_units};

#[tokio::main]
async fn main() {
    // Initialize the blockchain provider with the Polygon RPC URL.
    let provider = blockchain::init_blockchain(Some("https://polygon-rpc.com"))
        .await
        .expect("Failed to initialize Polygon provider");

    // Fetch the chain ID and current block number.
    let chain_id = provider.get_chainid().await.unwrap();
    let block_number = provider.get_block_number().await.unwrap();
    let gas_price = provider.get_gas_price().await.unwrap();

    // Print the retrieved information to the console.
    println!("Connected to Polygon (Chain ID: {})", chain_id);
    println!("Current block: {}", block_number);
    println!(
        "Gas price: {} Gwei",
        format_units(gas_price, "gwei").unwrap()
    );
}
