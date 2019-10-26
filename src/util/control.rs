#![allow(non_snake_case)]

use core::result::Result;
use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct Control {}

#[derive(Deserialize, Debug)]
pub struct ControlInfo {
    version: u32,
    protocolversion: u32,
    blocks: u32,
    timeoffset: i32,
    connections: u32,
    proxy: String,
    difficulty: f32,
    testnet: bool,
    paytxfee: u32,
    relayfee: f32,
    errors: String,
}

#[derive(Deserialize, Debug)]
pub struct NetworkInfo {
    version: u32,
    subversion: String,
    protocolversion: u32,
    localservices: String,
    localrelay: bool,
    timeoffset: i32,
    networkactive: bool,
    connections: u32,
    networks: Vec<Network>,
    relayfee: f32,
    excessutxocharge: u32,
    warnings: String,
}

#[derive(Deserialize, Debug)]
pub struct Network {
    name: String,
    limited: bool,
    reachable: bool,
    proxy: String,
    proxy_randomize_credentials: bool,
}

impl Control {
    pub fn get_info() -> Result<ControlInfo, Error> {
        let url: String = format!("{}control/getInfo", crate::MAINNET_BASE_URL);
        let s_slice: &str = &url[..];
        let info: ControlInfo = reqwest::get(s_slice)?.json()?;
        Ok(info)
    }

    pub fn get_network_info() -> Result<NetworkInfo, Error> {
        let url: String = format!("{}control/getNetworkInfo", crate::MAINNET_BASE_URL);
        let s_slice: &str = &url[..];
        let info: NetworkInfo = reqwest::get(s_slice)?.json()?;
        Ok(info)
    }
}
