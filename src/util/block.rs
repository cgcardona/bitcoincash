#![allow(non_snake_case)]

use core::result::Result;
use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct Block {}

#[derive(Deserialize, Debug)]
pub struct BlockDetails {
    hash: String,
    size: u32,
    height: u32,
    version: u32,
    merkleroot: String,
    tx: Vec<String>,
    time: u32,
    nonce: u32,
    bits: String,
    difficulty: f32,
    chainwork: String,
    confirmations: u32,
    previousblockhash: String,
    nextblockhash: String,
    reward: f32,
    isMainChain: bool,
    poolInfo: PoolInfo,
}

#[derive(Deserialize, Debug)]
pub struct PoolInfo {
    poolName: String,
    url: String,
}

impl Block {
    pub fn details_by_hash(block_hash: &str) -> Result<BlockDetails, Error> {
        let url: String = format!(
            "{}block/detailsByHash/{}",
            crate::MAINNET_BASE_URL,
            block_hash
        );
        let s_slice: &str = &url[..];
        let block_details: BlockDetails = reqwest::get(s_slice)?.json()?;
        Ok(block_details)
    }

    pub fn details_by_height(block_height: &u32) -> Result<BlockDetails, Error> {
        let url: String = format!(
            "{}block/detailsByHeight/{}",
            crate::MAINNET_BASE_URL,
            block_height
        );
        let s_slice: &str = &url[..];
        let block_details: BlockDetails = reqwest::get(s_slice)?.json()?;
        Ok(block_details)
    }
}
