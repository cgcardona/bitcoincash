extern crate bitcoin;
extern crate secp256k1;

use bitcoin::{
    network::constants::Network,
    util::{address::Address, key, key::PublicKey},
};
use secp256k1::{rand::thread_rng, Secp256k1};

fn main() {
    let s: Secp256k1<_> = Secp256k1::new();
    let public_key: PublicKey = key::PublicKey {
        compressed: true,
        key: s.generate_keypair(&mut thread_rng()).1,
    };
    // Generate pay-to-pubkey-hash address
    let address: Address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("Hello, {}!", address);
}
