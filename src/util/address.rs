use core::result::Result;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug)]
pub struct Address {}

const BASE_URL: &str = "https://rest.bitcoin.com/v2/address/";

#[derive(Deserialize, Debug)]
struct AddressDetails {
    legacyAddress: String,
    cashAddress: String,
    slpAddress: String,
    balance: u32,
    balanceSat: u32,
    totalReceived: f32,
    totalReceivedSat: u32,
    totalSent: f32,
    totalSentSat: u32,
    unconfirmedBalance: f32,
    unconfirmedBalanceSat: u32,
    unconfirmedTxApperances: u32,
    txApperances: u32,
    currentPage: u32,
    pagesTotal: u32,
    transactions: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct AddressUTXO {
    // TODO: Add UTXOs struct
    utxos: Vec<String>,
    legacyAddress: String,
    cashAddress: String,
    slpAddress: String,
    scriptPubKey: String,
}

#[derive(Deserialize, Debug)]
struct AddressUnconfirmed {
    // TODO: Add UTXOs struct
    utxos: Vec<String>,
    legacyAddress: String,
    cashAddress: String,
    slpAddress: String,
    scriptPubKey: String,
}

#[derive(Deserialize, Debug)]
struct AddressTransactions {
    pagesTotal: u32,
    legacyAddress: String,
    cashAddress: String,
    // TODO: Add Transactions struct
    // txs: Vec<String>,
    currentPage: u32,
}

impl Address {
    pub fn details(cash_address: &str) -> Result<(), Error> {
        let url: String = format!("{}details/{}", BASE_URL, cash_address);
        let s_slice: &str = &url[..];
        let address_details: AddressDetails = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_details);
        Ok(())
    }

    pub fn utxo(cash_address: &str) -> Result<(), Error> {
        let url: String = format!("{}utxo/{}", BASE_URL, cash_address);
        println!("{}", url);
        let s_slice: &str = &url[..];
        let address_utxo: AddressUTXO = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_utxo);
        Ok(())
    }

    pub fn unconfirmed(cash_address: &str) -> Result<(), Error> {
        let url: String = format!("{}unconfirmed/{}", BASE_URL, cash_address);
        println!("{}", url);
        let s_slice: &str = &url[..];
        let address_unconfirmed: AddressUnconfirmed = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", address_unconfirmed);
        Ok(())
    }

    pub fn transactions(cash_address: &str) -> Result<(), Error> {
        let url: String = format!("{}transactions/{}", BASE_URL, cash_address);
        let s_slice: &str = &url[..];
        let transactions: AddressTransactions = reqwest::get(s_slice)?.json()?;
        println!("{:#?}", transactions);
        Ok(())
    }
}
