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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1() {
        let s: &str = "hello world";
        assert_eq!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed", Crypto::sha1(s));
    }

    #[test]
    fn sha256() {
        let s: &str = "hello world";
        assert_eq!(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            Crypto::sha256(s)
        );
    }

    #[test]
    fn ripemd160() {
        let s: &str = "hello world";
        assert_eq!(
            "98c615784ccb5fe5936fbc0cbe9dfdb408d92f0f",
            Crypto::ripemd160(s)
        );
    }

    #[test]
    fn hash256() {
        let s: &str = "hello world";
        assert_eq!(
            "049da052634feb56ce6ec0bc648c672011edff1cb272b53113bbc90a8f00249c",
            Crypto::hash256(s)
        );
    }

    #[test]
    fn hash160() {
        let s: &str = "hello world";
        assert_eq!(
            "81ac71d673080916224ca67d0cb24dd2b63f17e4",
            Crypto::hash160(s)
        );
    }

    #[test]
    fn random_bytes() {
        // TODO test random_bytes
    }
}
