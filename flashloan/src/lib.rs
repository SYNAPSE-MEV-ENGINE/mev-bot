use ethers::{prelude::*, utils::parse_ether};
use std::sync::Arc;

pub struct FlashloanExecutor {
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    aave_pool: Address,
}

impl FlashloanExecutor {
    pub async fn new(rpc_url: &str, chain_id: u64) -> Result<Self> {
        let provider = Provider::<Http>::connect(rpc_url).await;
        let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(Self {
            client,
            aave_pool: "0x794a61358D6845594F94dc1DB02A252b5b4814aD".parse()?,
        })
    }

    pub async fn execute_arbitrage(&self, targets: Vec<Address>, data: Vec<Bytes>) {
        let loan_amount = parse_ether(1000).unwrap();

        let params = (
            targets,
            vec![loan_amount],
            vec![0],
            Address::zero(),
            data,
            0,
        );

        let tx = TransactionRequest::new()
            .to(self.aave_pool)
            .data(encode_function_data("flashLoan", params));

        self.client.send_transaction(tx, None).await.unwrap();
    }
}
