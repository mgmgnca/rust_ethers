use std::sync::Arc;
use ethers::contract::abigen;
use ethers::prelude::BlockNumber;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::Address;
use eyre::Result;


pub async fn test(client: Arc<Provider<Http>>, factory: Address, token_a: Address, token_b: Address) -> Result<()> {
    abigen!(
        IERC20,
        r#"[
            function getPair(address tokenA, address tokenB) external view returns (address pair)
        ]"#,
    );

    let contract = IERC20::new(factory, client.clone());
    let pool_address = contract.get_pair(token_a, token_b).await?;
    println!("pool_address :{pool_address}");

    Ok(())
}