// This example demonstrates how to connect to the Polygon blockchain and retrieve basic information.
// It initializes the blockchain provider and fetches the chain ID, current block number, and gas price.

use mev_core::blockchain;
use ethers::{prelude::*, utils::format_units};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a blockchain connection
    let connection = blockchain::BlockchainConnection::new(
        "http://localhost:8545",
        Chain::PolygonMumbai
    ).await?;

    // Get the provider
    let provider = connection.provider();
    
    // Get current block number
    let block_number = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);

    Ok(())
}
