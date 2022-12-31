use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::config::Configurator;
use crate::constants;

pub struct Alphabet {
    pub chars: Vec<char>,
    pub range: Uniform<usize>,
    pub rng: StdRng,
}

fn mixin_special_chars(config: &Configurator, chars: &mut Vec<char>) {
    if !config.args.get_flag("no_special_chars") {
        chars.append(&mut constants::SPECIAL_CHARS.to_vec());
    }
}

fn mixin_uppercase_letters(config: &Configurator, chars: &mut Vec<char>) {
    if !config.args.get_flag("no_uppercase") {
        for e in b'A'..b'Z' + 1 {
            chars.push(e as char);
        }
    }
}
fn mixin_lowercase_letters(config: &Configurator, chars: &mut Vec<char>) {
    if !config.args.get_flag("no_lowercase") {
        for e in b'a'..b'z' + 1 {
            chars.push(e as char);
        }
    }
}

fn mixin_numbers(config: &Configurator, chars: &mut Vec<char>) {
    if !config.args.get_flag("no_numbers") {
        for e in b'0'..b'9' + 1 {
            chars.push(e as char);
        }
    }
}

fn generate_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}

impl Alphabet {
    pub fn new(config: &Configurator) -> Alphabet {
        let mut chars: Vec<char> = Vec::with_capacity(constants::ESTIMATED_ALPHABET_CAPACITY);

        mixin_special_chars(config, &mut chars);
        mixin_uppercase_letters(config, &mut chars);
        mixin_lowercase_letters(config, &mut chars);
        mixin_numbers(config, &mut chars);

        let alphabet_length = chars.len();
        Alphabet {
            chars,
            range: Uniform::new(0, alphabet_length),
            rng: StdRng::from_seed(generate_seed()),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.rng.sample(self.range)]
    }
}
