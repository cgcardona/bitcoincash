extern crate crypto;

#[derive(Debug)]
pub struct Crypto {}

use self::crypto::{digest::Digest, ripemd160::Ripemd160, sha1::Sha1, sha2::Sha256};
use ring::rand::{SecureRandom, SystemRandom};

fn rng() -> &'static SecureRandom {
    use std::ops::Deref;

    lazy_static! {
        static ref RANDOM: SystemRandom = SystemRandom::new();
    }

    RANDOM.deref()
}

impl Crypto {
    pub fn sha1(data: &str) -> String {
        let mut hasher: Sha1 = Sha1::new();
        hasher.input_str(data);
        let hex: String = hasher.result_str();
        hex
    }

    pub fn sha256(data: &str) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.input_str(data);
        let hex: String = hasher.result_str();
        hex
    }

    pub fn ripemd160(data: &str) -> String {
        let mut hasher: Ripemd160 = Ripemd160::new();
        hasher.input_str(data);
        let hex: String = hasher.result_str();
        hex
    }

    pub fn hash256(data: &str) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.input_str(data);
        let hex: String = hasher.result_str();
        let mut hasher2: Sha256 = Sha256::new();
        hasher2.input_str(&hex);
        let hex2: String = hasher2.result_str();
        hex2
    }

    pub fn hash160(data: &str) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.input_str(data);
        let hex: String = hasher.result_str();
        let mut hasher2: Ripemd160 = Ripemd160::new();
        hasher2.input_str(&hex);
        let hex2: String = hasher2.result_str();
        hex2
    }

    pub fn random_bytes(size: usize) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = vec![0; size];
        rng().fill(&mut bytes).map_err(|e| e.to_string())?;
        Ok(bytes)
    }
}
