#[derive(Debug)]
pub struct Mnemonic {
    language: String,
    bits: u16,
}

impl Mnemonic {
    // TODO add shadow generate associated function which defaults to 256 bits
    pub fn generate(bits: u16) -> Mnemonic {
        Mnemonic {
            language: String::from("english"),
            bits: bits,
        }
    }

    pub fn from_entropy() {
        println!("from_entropy");
    }

    pub fn to_entropy() {
        println!("to_entropy");
    }

    pub fn validate() {
        println!("validate");
    }

    pub fn to_seed() {
        println!("to_seed");
    }

    pub fn word_lists() {
        println!("word_lists");
    }

    pub fn to_keypairs() {
        println!("to_keypairs");
    }

    pub fn find_nearest_word() {
        println!("find_nearest_word");
    }
}
