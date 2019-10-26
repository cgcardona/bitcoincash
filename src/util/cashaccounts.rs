use core::result::Result;
use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct CashAccounts {}

#[derive(Deserialize, Debug)]
pub struct Lookup {
    identifier: String,
    information: Information,
}

#[derive(Deserialize, Debug)]
pub struct Information {
    emoji: String,
    name: String,
    number: u32,
    collision: Collision,
    payment: Vec<Payment>,
}

#[derive(Deserialize, Debug)]
pub struct Collision {
    hash: String,
    count: u32,
    length: u32,
}

#[derive(Deserialize, Debug)]
pub struct Payment {
    // TODO add back type which is a reserved word in Rust
    // type: String,
    address: String,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    identifier: String,
    block: u32,
    results: Vec<Results>,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    transaction: String,
    inclusionProof: String,
}

#[derive(Deserialize, Debug)]
pub struct ReverseLookups {
    results: Vec<ReverseLookup>,
}

#[derive(Deserialize, Debug)]
pub struct ReverseLookup {
    accountEmoji: String,
    nameText: String,
    accountNumber: u32,
    accountHash: String,
    accountCollisionLength: u32,
    payloadType: u32,
    payloadAddress: String,
}

impl CashAccounts {
    pub fn lookup(account: &str, number: &str, collision: &str) -> Result<Lookup, Error> {
        let url: String = format!(
            "{}cashAccounts/lookup/{}/{}/{}",
            crate::MAINNET_BASE_URL,
            account,
            number,
            collision
        );
        let s_slice: &str = &url[..];
        let lookup: Lookup = reqwest::get(s_slice)?.json()?;
        Ok(lookup)
    }

    pub fn check(account: &str, number: &str) -> Result<Account, Error> {
        let url: String = format!(
            "{}cashAccounts/check/{}/{}",
            crate::MAINNET_BASE_URL,
            account,
            number
        );
        let s_slice: &str = &url[..];
        let account: Account = reqwest::get(s_slice)?.json()?;
        Ok(account)
    }

    pub fn reverse_lookup(cash_address: &str) -> Result<ReverseLookups, Error> {
        let url: String = format!(
            "{}cashAccounts/reverselookup/{}",
            crate::MAINNET_BASE_URL,
            cash_address
        );
        let s_slice: &str = &url[..];
        let reverse_lookups: ReverseLookups = reqwest::get(s_slice)?.json()?;
        Ok(reverse_lookups)
    }
}
