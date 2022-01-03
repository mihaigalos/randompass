use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::config::Configurator;
use crate::constants;

pub struct Alphabet {
    chars: Vec<char>,
    range: Uniform<usize>,
    rng: StdRng,
}

fn mixin_special_chars(config: &Configurator, chars: &mut Vec<char>) {
    if !config.cli_args.is_present("no_special_chars") {
        chars.append(&mut constants::SPECIAL_CHARS.to_vec());
    }
}

fn mixin_uppercase_letters(config: &Configurator, chars: &mut Vec<char>) {
    if !config.cli_args.is_present("no_uppercase") {
        for e in 'A' as u8..'Z' as u8 + 1 {
            chars.push(e as char);
        }
    }
}
fn mixin_lowercase_letters(config: &Configurator, chars: &mut Vec<char>) {
    if !config.cli_args.is_present("no_lowercase") {
        for e in 'a' as u8..'z' as u8 + 1 {
            chars.push(e as char);
        }
    }
}

fn mixin_numbers(config: &Configurator, chars: &mut Vec<char>) {
    if !config.cli_args.is_present("no_numbers") {
        for e in '0' as u8..'9' as u8 + 1 {
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
