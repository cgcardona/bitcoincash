use core::result::Result;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SLP {}

const BASE_URL: &str = "https://rest.bitcoin.com/v2/slp/";

#[derive(Deserialize, Debug)]
struct SLPTokens {
    tokens: (SLPToken),
}

#[derive(Deserialize, Debug)]
struct SLPToken {
    decimals: u8,
    timestamp: String,
    versionType: u8,
    documentUri: String,
    symbol: String,
    name: String,
    containsBaton: bool,
    id: String,
    documentHash: String,
    initialTokenQty: u32,
    blockCreated: u32,
    blockLastActiveSend: u32,
    blockLastActiveMint: String,
    txnsSinceGenesis: u32,
    validAddresses: u32,
    totalMinted: u32,
    totalBurned: u32,
    circulatingSupply: u32,
    mintingBatonStatus: String,
    timestampUnix: u32,
}

#[derive(Deserialize, Debug)]
struct SLPBalances {
    balances: (SLPBalance),
}

#[derive(Deserialize, Debug)]
struct SLPBalance {
    tokenId: String,
    balance: u32,
    balanceString: String,
    slpAddress: String,
    decimalCount: u8,
}

#[derive(Deserialize, Debug)]
struct Balance {
    tokenId: String,
    balance: u32,
    balanceString: String,
    slpAddress: String,
    legacyAddress: String,
    cashAddress: String,
}

#[derive(Deserialize, Debug)]
struct SLPConversion {
    slpAddress: String,
    cashAddress: String,
    legacyAddress: String,
}

#[derive(Deserialize, Debug)]
struct TokenBalances {
    balances: TokenBalance,
}

#[derive(Deserialize, Debug)]
struct TokenBalance {
    tokenBalance: u32,
    tokenBalanceString: String,
    slpAddress: String,
    tokenId: String,
}

impl SLP {
    pub fn list() -> Result<(), Error> {
        let url: String = format!("{}list", BASE_URL);
        let s_slice: &str = &url[..];
        let tokens: SLPTokens = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", tokens);
        Ok(())
    }

    // pub fn list(token_id: &str) -> Result<(), Error> {
    //     let url: String = format!("{}list/{}", BASE_URL, slp_address);
    //     let s_slice: &str = &url[..];
    //     let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
    //     println!("{:#?}", address_balances);
    //     Ok(())
    // }

    pub fn convert(slp_address: &str) -> Result<(), Error> {
        let url: String = format!("{}convert/{}", BASE_URL, slp_address);
        let s_slice: &str = &url[..];
        let conversion: SLPConversion = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", conversion);
        Ok(())
    }

    pub fn balances_for_address(slp_address: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForAddress/{}", BASE_URL, slp_address);
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_balances);
        Ok(())
    }

    pub fn balances_for_token(token_id: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForToken/{}", BASE_URL, token_id);
        println!("URL: {:#?}", url);
        let s_slice: &str = &url[..];
        let token_balances: TokenBalances = reqwest::get(s_slice)?.json()?;
        // TODO: properly capture this returned data
        println!("Token Balances: {:?}", token_balances);
        Ok(())
    }

    pub fn balance(slp_address: &str, token_id: &str) -> Result<(), Error> {
        let url: String = format!("{}balance/{}/{}", BASE_URL, slp_address, token_id);
        let s_slice: &str = &url[..];
        let balance: Balance = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", balance);
        Ok(())
    }

    pub fn validate_txid(txid: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForAddress/{}", BASE_URL, txid);
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_balances);
        Ok(())
    }

    pub fn token_stats(token_id: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForAddress/{}", BASE_URL, token_id);
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_balances);
        Ok(())
    }

    pub fn transaction_details(txid: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForAddress/{}", BASE_URL, txid);
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_balances);
        Ok(())
    }

    pub fn transactions(token_id: &str, slp_address: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForAddress/{}", BASE_URL, slp_address);
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_balances);
        Ok(())
    }

    pub fn burn_total(txid: &str) -> Result<(), Error> {
        let url: String = format!("{}balancesForAddress/{}", BASE_URL, txid);
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_balances);
        Ok(())
    }
}
