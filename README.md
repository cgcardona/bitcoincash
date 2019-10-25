# bitcoincash

Rust crate for Bitcoin Cash

## Getting started

### Installation

First add `bitcoincash` to your Cargo.toml file's `dependencies` block.

```
[dependencies]
bitcoincash = "0.0.1"
```

## Usage

```rs
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
```

Then when you run it

```
cargo run
   Compiling bitcoincash v0.0.1 (/Users/username/bitcoincash)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/bitcoincash`
Hello, 1HZ3X63fM5mWJbMpwhEXVFysXoQgxCjCXn!
```
