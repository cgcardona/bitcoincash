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
extern crate ring;

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

    let s: &str = "hello world";
    let c: String = util::Crypto::hash160(s);
    println!("Crypto {:#?}", c);

    let r: Result<Vec<u8>, String> = util::Crypto::random_bytes(128);
    // println!("Crypto {:#?}", r);
}
