use crate::alphabet::Alphabet;
use crate::config::Configurator;
use crate::constants;

pub struct Password {}

impl Password {
    pub fn generate(config: Configurator) -> String {
        let mut watchdog: usize = constants::MAX_CONCERGENCE_ITERATIONS;
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
                if watchdog == 0 {
                    break;
                }
            }
            watchdog = watchdog - 1;
        }
        return "ERROR: Cannot generate password after MAX iterations. Consider lowering constraints.".to_string();
    }

    fn validate(config: &Configurator, pass: String) -> bool {
        Password::validate_special_chars(config, pass.clone())
            && Password::validate_uppercase(config, pass.clone())
            && Password::validate_lowercase(config, pass.clone())
            && Password::validate_numbers(config, pass)
    }

    fn validate_special_chars(config: &Configurator, pass: String) -> bool {
        let mut ok_special_chars = false;
        for e in constants::SPECIAL_CHARS.to_vec().iter() {
            if config.cli_args.is_present("no_special_chars") && pass.contains(&e.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_special_chars")
                && pass.contains(&e.to_string())
            {
                ok_special_chars = true;
            }
        }
        if !ok_special_chars {
            return false;
        }
        true
    }

    fn validate_uppercase(config: &Configurator, pass: String) -> bool {
        let mut ok_uppercase = false;
        for e in 'A' as u8..'Z' as u8 + 1 {
            if config.cli_args.is_present("no_uppercase") && pass.contains(&e.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_uppercase") && pass.contains(&e.to_string()) {
                ok_uppercase = true;
            }
        }
        if !ok_uppercase {
            return false;
        }
        true
    }

    fn validate_lowercase(config: &Configurator, pass: String) -> bool {
        let mut ok_lowercase = false;
        for e in 'a' as u8..'z' as u8 + 1 {
            if config.cli_args.is_present("no_lowercase") && pass.contains(&e.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_lowercase") && pass.contains(&e.to_string()) {
                ok_lowercase = true;
            }
        }
        if !ok_lowercase {
            return false;
        }
        true
    }

    fn validate_numbers(config: &Configurator, pass: String) -> bool {
        let mut ok_numbers = false;
        for e in '0' as u8..'9' as u8 + 1 {
            if config.cli_args.is_present("no_numbers") && pass.contains(&e.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_numbers") && pass.contains(&e.to_string()) {
                ok_numbers = true;
            }
        }
        if !ok_numbers {
            return false;
        }
        true
    }
}

#[test]
fn test_pass_generate_works_when_typical() {
    use clap::App;
    let arg_vec = vec!["randompass"];
    let cli_args = App::new("myprog").get_matches_from(arg_vec);

    let actual = Password::generate(Configurator { cli_args });

    assert!(actual.len() == constants::DEFAULT_PASS_LEN);
}
