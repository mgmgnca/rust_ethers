use std::sync::Arc;
use std::time::Instant;
use ethers::prelude::*;
use ethers::types::Address;
use eyre::Result;
use rust_ethers::eg;

const RPC_URL: &str = "https://polygon-bor-rpc.publicnode.com";
const MY_ADDRESS: &str = "0xD9A489344E30606fd72B941B910Bb98ea3194B4B";
const USDT_ADDRESS: &str = "0xc2132d05d31c914a87c6611c10748aeb04b58e8f";

const WETH9_ADDRESS: &str = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619";
const UNISWAPV2_FACTORY: &str = "0x9e5A52f57b3038F1B8EeE45F28b3C1967e22799C";

#[tokio::main]
async fn main() -> Result<()> {

    let time = Instant::now();
    // Provider
    let provider: Provider<Http>= Provider::<Http>::try_from(RPC_URL)?;
    let client: Arc<Provider<Http>> = Arc::new(provider);
    let my_address = MY_ADDRESS.parse::<Address>()?;
    let usdt_address = USDT_ADDRESS.parse::<Address>()?;
    let eth_address = WETH9_ADDRESS.parse::<Address>()?;
    let uniswap_v2_factory = UNISWAPV2_FACTORY.parse::<Address>()?;

    eg::uniswapv2::test(client.clone(),uniswap_v2_factory, usdt_address, eth_address).await?;
    eg::singlecall::test(client.clone(), my_address, usdt_address).await?;
    eg::multicall::test(client.clone(), my_address, usdt_address).await?;
    eg::create2::test().await?;

    println!("{:?}", time.elapsed());
    Ok(())
}