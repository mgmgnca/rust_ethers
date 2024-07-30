// use super::address;
// pub enum ChainId {
//     POLYGON = 137
// }
// pub struct Token {
//     address: String,
//     chain_id: ChainId,
//     decimals: u8,
//     symbol: String,
//     name: String,
// }
//
// pub const WNATIVE: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::WNATIVE_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "WMATIC".to_string(),
//     name: "Wrapped Matic".to_string(),
// };
// pub const WETH9: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::WETH9_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "WETH".to_string(),
//     name: "Wrapped Ether".to_string(),
// };
// const WBTC: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::WBTC_ADDRESS.to_string(),
//     decimals: 8,
//     symbol: "WBTC".to_string(),
//     name: "Wrapped BTC".to_string(),
// };
// const USDC: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::USDC_ADDRESS.to_string(),
//     decimals: 6,
//     symbol: "USDC".to_string(),
//     name: "USD Coin".to_string(),
// };
// const USDT: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::USDT_ADDRESS.to_string(),
//     decimals: 6,
//     symbol: "USDT".to_string(),
//     name: "Tether USD".to_string(),
// };
// const DAI: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::DAI_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "DAI".to_string(),
//     name: "Dai Stablecoin".to_string(),
// };
// const FRAX: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::FRAX_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "FRAX".to_string(),
//     name: "FRAX".to_string(),
// };
// const QUICK: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::QUICK_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "QUICK".to_string(),
//     name: "Quickswap".to_string(),
// };
// const LINK: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::LINK_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "LINK".to_string(),
//     name: "ChainLink Token".to_string(),
// };
// const AAVE: Token = Token {
//     chain_id: ChainId::POLYGON,
//     address: address::AAVE_ADDRESS.to_string(),
//     decimals: 18,
//     symbol: "AAVE".to_string(),
//     name: "Aave Token".to_string(),
// };
// const USDE: Token = Token {
//     address: address::USDE_ADDRESS.to_string(),
//     chain_id: ChainId::POLYGON,
//     decimals: 6,
//     symbol: "USD Coin".to_string(),
//     name: "USDC.e".to_string(),
// };
//
// pub const BASES_TO_CHECK_TRADES_AGAINST : Vec<Token>  = vec![
//     WNATIVE,
//     WETH9,
//     WBTC,
//     USDC,
//     USDT,
//     DAI,
//     FRAX,
//     QUICK,
//     LINK,
//     AAVE,
//     USDE
// ];
//
