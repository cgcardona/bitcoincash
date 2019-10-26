#![allow(non_snake_case)]

use core::result::Result;
use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct Transaction {}

#[derive(Deserialize, Debug)]
pub struct Vin {
    // coinbase: String,
    sequence: u32,
    n: u32,
}

#[derive(Deserialize, Debug)]
pub struct Vout {
    value: String,
    n: u32,
    // scriptPubKey: ScriptPubKey,
    // spentTxId: String,
    // spentIndex: u32,
    // spentHeight: u32,
}

#[derive(Deserialize, Debug)]
pub struct ScriptPubKey {
    hex: String,
    asm: String,
    // addresses: Vec<String>,
    // TODO: add back type key (type is a reserved keyword in Rust)
    // TODO: add optional keys
    cashAddrs: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionDetails {
    txid: String,
    version: u8,
    locktime: u32,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    blockhash: String,
    blockheight: u32,
    confirmations: u32,
    time: u64,
    blocktime: u64,
    // isCoinBase: bool,
    valueOut: f32,
    size: u32,
}

impl Transaction {
    pub fn get_mining_info(txid: &str) -> Result<TransactionDetails, Error> {
        let url: String = format!("{}transaction/details/{}", crate::MAINNET_BASE_URL, txid);
        println!("{}", url);
        let s_slice: &str = &url[..];
        let transaction_details: TransactionDetails = reqwest::get(s_slice)?.json()?;
        Ok(transaction_details)
    }
}
