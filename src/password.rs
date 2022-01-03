use crate::alphabet::Alphabet;
use crate::config::Configurator;
use crate::constants;

pub struct Password {}

impl Password {
    pub fn generate(config: Configurator) -> String {
        let mut max_iterations: usize = constants::MAX_CONCERGENCE_ITERATIONS;
        loop {
            let mut alphabet = Alphabet::new(&config);
            let length: usize = config
                .cli_args
                .value_of("length")
                .unwrap_or(&constants::DEFAULT_PASS_LEN.to_string())
                .parse()
                .unwrap();

            let mut pass = String::new();
            for _ in 0..length {
                pass.push(alphabet.get_char());
            }

            if Password::validate(&config, pass.clone()) {
                return pass;
            } else {
                if max_iterations == 0 {
                    break;
                }
            }
            max_iterations = max_iterations - 1;
        }
        return "ERROR: Cannot generate password after MAX iterations. Consider lowering constraints.".to_string();
    }

    fn validate(config: &Configurator, pass: String) -> bool {
        if config.cli_args.is_present("no_special_chars") {
            for e in constants::SPECIAL_CHARS.to_vec().iter() {
                if pass.contains(&e.to_string()) {
                    return false;
                }
            }
        }
        if config.cli_args.is_present("no_uppercase") {
            for e in 'A' as u8..'Z' as u8 + 1 {
                if pass.contains(&e.to_string()) {
                    return false;
                }
            }
        }
        if config.cli_args.is_present("no_lowercase") {
            for e in 'a' as u8..'z' as u8 + 1 {
                if pass.contains(&e.to_string()) {
                    return false;
                }
            }
        }

        if config.cli_args.is_present("no_numbers") {
            for e in '0' as u8..'9' as u8 + 1 {
                if pass.contains(&e.to_string()) {
                    return false;
                }
            }
        }
        true
    }
}
