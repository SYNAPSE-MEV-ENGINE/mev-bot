#[tokio::main]
async fn main() {
    let fork_url = "https://polygon-mainnet.g.alchemy.com/v2/demo";
    let fork_block = 50_000_000;

    let fork = Fork::new(fork_url, fork_block);
    let mut evm = Evm::new(fork);

    let sandwich = SandwichModel::new(fork);
    let profit = sandwich.calculate_profit(target_tx, gas_cost);

    println!("Expected profit: {:?}", profit);
}
