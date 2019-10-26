#![allow(non_snake_case)]
// TODO: remove all non_snake_case attributes
use core::result::Result;
use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct SLP {}

#[derive(Deserialize, Debug)]
pub struct SLPTokens {
    tokens: (SLPToken),
}

#[derive(Deserialize, Debug)]
pub struct SLPToken {
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
pub struct SLPBalances {
    balances: (SLPBalance),
}

#[derive(Deserialize, Debug)]
pub struct SLPBalance {
    tokenId: String,
    balance: u32,
    balanceString: String,
    slpAddress: String,
    decimalCount: u8,
}

#[derive(Deserialize, Debug)]
pub struct Balance {
    tokenId: String,
    balance: u32,
    balanceString: String,
    slpAddress: String,
    legacyAddress: String,
    cashAddress: String,
}

#[derive(Deserialize, Debug)]
pub struct SLPConversion {
    slpAddress: String,
    cashAddress: String,
    legacyAddress: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenBalances {
    balances: TokenBalance,
}

#[derive(Deserialize, Debug)]
pub struct TokenBalance {
    tokenBalance: u32,
    tokenBalanceString: String,
    slpAddress: String,
    tokenId: String,
}

#[derive(Deserialize, Debug)]
pub struct SLPValidate {
    txid: String,
    valid: bool,
}

#[derive(Deserialize, Debug)]
pub struct SLPTokenStats {
    decimals: u8,
    timestamp: String,
    versionType: u8,
    documentUri: String,
    symbol: String,
    name: String,
    containsBaton: bool,
    id: String,
    // documentHash: String,
    initialTokenQty: u32,
    blockCreated: u32,
    blockLastActiveSend: u32,
    // blockLastActiveMint: u32,
    txnsSinceGenesis: u32,
    validAddresses: u32,
    totalMinted: u32,
    totalBurned: f32,
    circulatingSupply: f32,
    mintingBatonStatus: String,
    timestampUnix: u32,
}

#[derive(Deserialize, Debug)]
pub struct SLPTxDetails {
    txid: String,
    version: u8,
    locktime: u32,
}

#[derive(Deserialize, Debug)]
pub struct SLPTransactions {
    transactions: SLPTransaction,
}

#[derive(Deserialize, Debug)]
pub struct SLPTransaction {
    txid: String,
}

#[derive(Deserialize, Debug)]
pub struct SLPBurnTotal {
    transactionId: String,
    inputTotal: u32,
    outputTotal: u32,
    burnTotal: i32,
}

impl SLP {
    pub fn list() -> Result<SLPTokens, Error> {
        let url: String = format!("{}slp/list", MAINNET_BASE_URL);
        let s_slice: &str = &url[..];
        let tokens: SLPTokens = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", tokens);
        Ok(tokens)
    }

    // pub fn list(token_id: &str) -> Result<(), Error> {
    //     let url: String = format!("{}list/{}", BASE_URL, slp_address);
    //     let s_slice: &str = &url[..];
    //     let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
    //     println!("{:#?}", address_balances);
    //     Ok(())
    // }

    pub fn convert(slp_address: &str) -> Result<SLPConversion, Error> {
        let url: String = format!("{}slp/convert/{}", crate::MAINNET_BASE_URL, slp_address);
        let s_slice: &str = &url[..];
        let conversion: SLPConversion = reqwest::get(s_slice)?.json()?;
        Ok(conversion)
    }

    pub fn balances_for_address(slp_address: &str) -> Result<SLPBalances, Error> {
        let url: String = format!(
            "{}slp/balancesForAddress/{}",
            crate::MAINNET_BASE_URL,
            slp_address
        );
        let s_slice: &str = &url[..];
        let address_balances: SLPBalances = reqwest::get(s_slice)?.json()?;
        Ok(address_balances)
    }

    pub fn balances_for_token(token_id: &str) -> Result<TokenBalances, Error> {
        let url: String = format!(
            "{}slp/balancesForToken/{}",
            crate::MAINNET_BASE_URL,
            token_id
        );
        println!("URL: {:#?}", url);
        let s_slice: &str = &url[..];
        let token_balances: TokenBalances = reqwest::get(s_slice)?.json()?;
        // TODO: properly capture this returned data
        Ok(token_balances)
    }

    pub fn balance(slp_address: &str, token_id: &str) -> Result<Balance, Error> {
        let url: String = format!(
            "{}slp/balance/{}/{}",
            crate::MAINNET_BASE_URL,
            slp_address,
            token_id
        );
        let s_slice: &str = &url[..];
        let balance: Balance = reqwest::get(s_slice)?.json()?;
        Ok(balance)
    }

    pub fn validate_txid(txid: &str) -> Result<SLPValidate, Error> {
        let url: String = format!("{}slp/validateTxid/{}", crate::MAINNET_BASE_URL, txid);
        let s_slice: &str = &url[..];
        let valid: SLPValidate = reqwest::get(s_slice)?.json()?;
        Ok(valid)
    }

    pub fn token_stats(token_id: &str) -> Result<SLPTokenStats, Error> {
        let url: String = format!("{}slp/tokenStats/{}", crate::MAINNET_BASE_URL, token_id);
        let s_slice: &str = &url[..];
        let token_stats: SLPTokenStats = reqwest::get(s_slice)?.json()?;
        Ok(token_stats)
    }

    pub fn transaction_details(txid: &str) -> Result<SLPTxDetails, Error> {
        let url: String = format!("{}slp/txDetails/{}", crate::MAINNET_BASE_URL, txid);
        let s_slice: &str = &url[..];
        let tx_details: SLPTxDetails = reqwest::get(s_slice)?.json()?;
        Ok(tx_details)
    }

    pub fn transactions(token_id: &str, slp_address: &str) -> Result<SLPTransactions, Error> {
        let url: String = format!(
            "{}slp/transactions/{}",
            crate::MAINNET_BASE_URL,
            slp_address
        );
        let s_slice: &str = &url[..];
        let transactions: SLPTransactions = reqwest::get(s_slice)?.json()?;
        Ok(transactions)
    }

    // TODO: why is this burnTotal a negative value: https://rest.bitcoin.com/v2/slp/burnTotal/df808a41672a0a0ae6475b44f272a107bc9961b90f29dc918d71301f24fe92fb
    pub fn burn_total(txid: &str) -> Result<SLPBurnTotal, Error> {
        let url: String = format!("{}slp/burnTotal/{}", crate::MAINNET_BASE_URL, txid);
        let s_slice: &str = &url[..];
        let burn_total: SLPBurnTotal = reqwest::get(s_slice)?.json()?;
        Ok(burn_total)
    }
}
