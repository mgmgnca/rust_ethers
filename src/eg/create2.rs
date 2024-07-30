use ethers::abi::{self, Token};
use ethers::types::{Address, H256};
use ethers::utils;
use eyre::Result;

pub async fn test () -> Result<()> {


    let factory = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;
    let token_a = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>()?;
    let token_b = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>()?;

    let encoded = abi::encode_packed(&[Token::Address(token_a), Token::Address(token_b)])?;
    let salt = utils::keccak256(encoded);
    let init_code_hash: H256 = "0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f".parse()?;
    let pair = utils::get_create2_address_from_hash(factory, salt, init_code_hash);

    println!("pair Address: {:#?}", pair);
    let weth_usdc = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;
    assert_eq!(pair, weth_usdc);

    Ok(())
}