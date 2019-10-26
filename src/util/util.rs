#![allow(non_snake_case)]

use core::result::Result;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Util {}

const BASE_URL: &str = "https://rest.bitcoin.com/v2/util/";

#[derive(Deserialize, Debug)]
pub struct Validation {
    isvalid: bool,
    address: String,
    scriptPubKey: String,
    ismine: bool,
    iswatchonly: bool,
    isscript: bool,
}

impl Util {
    pub fn validate_address(cash_address: &str) -> Result<Validation, Error> {
        let url: String = format!("{}validateAddress/{}", BASE_URL, cash_address);
        let s_slice: &str = &url[..];
        let info: Validation = reqwest::get(s_slice)?.json()?;
        Ok(info)
    }
}
