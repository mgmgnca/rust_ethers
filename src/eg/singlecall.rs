use std::sync::Arc;
use ethers::contract::abigen;
use ethers::prelude::BlockNumber;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::Address;
use eyre::Result;


pub async fn test(client: Arc<Provider<Http>>, my_address: Address, usdt_address: Address) -> Result<()> {
    abigen!(
        IERC20,
        r#"[
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
    );

    let block_number = client.get_block_number().await?;
    println!("block_number: {block_number}");

    let latest_block = client.get_block(BlockNumber::Latest).await?.unwrap();
    println!("latest_block: {:#?}", latest_block.number.unwrap());

    let matic_balance = client.get_balance(my_address, None).await?;
    println!("matic_balance: {matic_balance}");

    let contract = IERC20::new(usdt_address, client.clone());
    let usdt_balance = contract.balance_of(my_address).await?;
    println!("usdt_balance :{usdt_balance}");



    Ok(())
}