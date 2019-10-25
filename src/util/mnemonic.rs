#[derive(Debug)]
pub struct Mnemonic {
    language: String,
    bits: u16,
    phrase: String,
}

// TODO create Bits enum w/ different bitlength variants.

impl Mnemonic {
    // TODO add shadow generate associated function which defaults to 256 bits
    pub fn generate(bits: u16) -> Mnemonic {
        let m: String = match bits {
            128 => String::from("slight memory siren then keen girl limb online orient myth honey merry"),
            160 => String::from("fuel mesh color gown relief treat enroll web transfer consider wise clay length tent simple"),
            192 => String::from("outdoor course hire install kick danger utility category current palm hybrid ankle embody worry ridge leg fever bargain"),
            224 => String::from("mix derive guilt grass camera camp era display flower enhance web eagle curious fossil cave gravity defense noble width entire shy"),
            256 => String::from("awful rally wealth nose original city shine follow kit drama cage gentle lumber oppose faint pudding cup circle urge patient lion chest glance wedding"),
            _ => String::from(""),
        };

        Mnemonic {
            language: String::from("english"),
            bits: bits,
            phrase: m,
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
