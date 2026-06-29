use rand::distr::Uniform;
use rand::rngs::ThreadRng;
use rand::RngExt;

use crate::config::Configurator;
use crate::constants;

pub struct Alphabet {
    pub chars: Vec<char>,
    pub range: Uniform<usize>,
    pub rng: ThreadRng,
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
            range: Uniform::new(0, alphabet_length).expect("alphabet is non-empty"),
            rng: rand::rng(),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.rng.sample(self.range)]
    }
}
