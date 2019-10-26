//! # Rust Bitcoin Cash Library
//!
//! This is a library for which supports the Bitcoin Cash network protocol and associated
//! primitives. It is designed for Rust programs built to work with the Bitcoin Cash network.
//!
//! It is also written entirely in Rust to illustrate the benefits of strong type
//! safety, including ownership and lifetime, for financial and/or cryptographic
//! software.

// extern crate bitcoin;
// extern crate secp256k1;

// use bitcoin::{
//     network::constants::Network,
//     util::{address::Address, key, key::PublicKey},
// };
// use secp256k1::{rand::thread_rng, Secp256k1};

// fn main() {
//     let s: Secp256k1<_> = Secp256k1::new();
//     let public_key: PublicKey = key::PublicKey {
//         compressed: true,
//         key: s.generate_keypair(&mut thread_rng()).1,
//     };
//     // Generate pay-to-pubkey-hash address
//     let address: Address = Address::p2pkh(&public_key, Network::Bitcoin);
//     println!("Hello, {}!", address);
// }

// #[derive(Debug)]
// struct Mnemonic {
//     language: String,
//     bits: u16,
// }
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate ring;
extern crate serde;
extern crate serde_derive;

mod util;

fn main() {
    // let bits: u16 = 256;
    // let m: util::Mnemonic = util::Mnemonic::generate(bits);
    // println!("Hello {:#?}", m);
    // util::Mnemonic::from_entropy();
    // util::Mnemonic::to_entropy();
    // util::Mnemonic::validate();
    // util::Mnemonic::to_seed();
    // util::Mnemonic::word_lists();
    // util::Mnemonic::to_keypairs();
    // util::Mnemonic::to_keypairs();

    // let s: &str = "hello world";
    // let c: String = util::Crypto::hash160(s);
    // println!("Crypto {:#?}", c);

    // let r: Result<Vec<u8>, String> = util::Crypto::random_bytes(128);
    // println!("Crypto {:#?}", r);

    // let cash_address: &str = "simpleledger:qz9tzs6d5097ejpg279rg0rnlhz546q4fsnck9wh5m";
    // let r = util::SLP::balances_for_address(&cash_address);

    // let token_id: &str = "df808a41672a0a0ae6475b44f272a107bc9961b90f29dc918d71301f24fe92fb";
    // let t = util::SLP::balances_for_token(&token_id);

    // let cash_address: &str = "simpleledger:qz9tzs6d5097ejpg279rg0rnlhz546q4fsnck9wh5m";
    // let token_id: &str = "df808a41672a0a0ae6475b44f272a107bc9961b90f29dc918d71301f24fe92fb";
    // let t = util::SLP::balance(&cash_address, &token_id);

    // let token_id: &str = "df808a41672a0a0ae6475b44f272a107bc9961b90f29dc918d71301f24fe92fb";
    // let t = util::SLP::burn_total(&token_id);
    // println!("{:#?}", t);

    // let block_height: &str = "000000000000000005e14d3f9fdfb70745308706615cfa9edca4f4558332b201";
    // let b = util::Block::details_by_hash(&block_height);
    // println!("{:#?}", b);

    // let block_height: u32 = 500000;
    // let b = util::Block::details_by_height(&block_height);
    // println!("{:#?}", b);

    let cash_address: &str = "bitcoincash:qzs02v05l7qs5s24srqju498qu55dwuj0cx5ehjm2c";
    let b = util::Util::validate_address(cash_address);
    println!("{:#?}", b);
}
