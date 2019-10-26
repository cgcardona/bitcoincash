#![allow(non_snake_case)]

use core::result::Result;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Blockchain {}

const BASE_URL: &str = "https://rest.bitcoin.com/v2/blockchain/";

#[derive(Deserialize, Debug)]
pub struct BlockchainInfo {
    chain: String,
    blocks: u32,
    headers: u32,
    bestblockhash: String,
    difficulty: f64,
    mediantime: u32,
    verificationprogress: f32,
    chainwork: String,
    size_on_disk: u64,
    pruned: bool,
    warnings: String,
    softforks: Vec<Softfork>,
}

#[derive(Deserialize, Debug)]
pub struct Softfork {
    id: String,
    version: u8,
}

#[derive(Deserialize, Debug)]
pub struct BlockHeader {
    hash: String,
    confirmations: u32,
    height: u32,
    version: u32,
    versionHex: String,
    merkleroot: String,
    time: u32,
    mediantime: u32,
    nonce: u32,
    bits: String,
    difficulty: f32,
    chainwork: String,
    previousblockhash: String,
    nextblockhash: String,
}

#[derive(Deserialize, Debug)]
pub struct ChainTips {
    chaintips: Vec<ChainTip>,
}

#[derive(Deserialize, Debug)]
pub struct ChainTip {
    height: u32,
    hash: String,
    branchlen: u32,
    status: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryError {
    error: String,
}

#[derive(Deserialize, Debug)]
pub struct MempoolInfo {
    size: u32,
    bytes: u32,
    usage: u32,
    maxmempool: u32,
    mempoolminfee: u32,
}

#[derive(Deserialize, Debug)]
pub struct RawMempool {}

impl Blockchain {
    pub fn get_best_block_hash() -> Result<String, Error> {
        let url: String = format!("{}getBestBlockHash", BASE_URL);
        let s_slice: &str = &url[..];
        let block_hash: String = reqwest::get(s_slice)?.json()?;
        Ok(block_hash)
    }

    pub fn get_blockchain_info() -> Result<BlockchainInfo, Error> {
        let url: String = format!("{}getBlockchainInfo", BASE_URL);
        let s_slice: &str = &url[..];
        let blockchain_info: BlockchainInfo = reqwest::get(s_slice)?.json()?;
        Ok(blockchain_info)
    }

    pub fn get_block_count() -> Result<u32, Error> {
        let url: String = format!("{}getBlockCount", BASE_URL);
        let s_slice: &str = &url[..];
        let block_count: u32 = reqwest::get(s_slice)?.json()?;
        Ok(block_count)
    }

    pub fn get_block_header(block_hash: &str) -> Result<String, Error> {
        // TODO: Add query string params
        let url: String = format!("{}getBlockHeader/{}", BASE_URL, block_hash);
        let s_slice: &str = &url[..];
        let block_header: String = reqwest::get(s_slice)?.json()?;
        Ok(block_header)
    }

    pub fn get_chain_tips() -> Result<ChainTips, Error> {
        // TODO: Get this working properly
        let url: String = format!("{}getChainTips", BASE_URL);
        let s_slice: &str = &url[..];
        let chain_tips: ChainTips = reqwest::get(s_slice)?.json()?;
        Ok(chain_tips)
    }

    pub fn get_difficulty() -> Result<f32, Error> {
        let url: String = format!("{}getDifficulty", BASE_URL);
        let s_slice: &str = &url[..];
        let difficulty: f32 = reqwest::get(s_slice)?.json()?;
        Ok(difficulty)
    }

    pub fn get_mempool_entry(txid: &str) -> Result<QueryError, Error> {
        // TODO: Add query string and match for Ok/Err
        let url: String = format!("{}getMempoolEntry/{}", BASE_URL, txid);
        let s_slice: &str = &url[..];
        let mempool_entry: QueryError = reqwest::get(s_slice)?.json()?;
        Ok(mempool_entry)
    }

    pub fn get_mempool_info() -> Result<MempoolInfo, Error> {
        let url: String = format!("{}getMempoolInfo", BASE_URL);
        let s_slice: &str = &url[..];
        let mempool_info: MempoolInfo = reqwest::get(s_slice)?.json()?;
        Ok(mempool_info)
    }

    pub fn get_raw_mempool() -> Result<RawMempool, Error> {
        // TODO: Add query string and match for Ok/Err
        let url: String = format!("{}getMempoolInfo", BASE_URL);
        let s_slice: &str = &url[..];
        let raw_mempool: RawMempool = reqwest::get(s_slice)?.json()?;
        Ok(raw_mempool)
    }
}
