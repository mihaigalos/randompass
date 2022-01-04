use crate::alphabet::Alphabet;
use crate::config::Configurator;
use crate::constants;

pub struct Password {}

impl Password {
    pub fn generate(config: &Configurator) -> String {
        let mut watchdog: usize = constants::MAX_CONVERGENCE_ITERATIONS;
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
        if !ok_special_chars && config.cli_args.is_present("no_special_chars") {
            ok_special_chars = true;
        }
        ok_special_chars
    }

    fn validate_uppercase(config: &Configurator, pass: String) -> bool {
        let mut ok_uppercase = false;
        for e in 'A' as u8..'Z' as u8 + 1 {
            let c = e as char;
            if config.cli_args.is_present("no_uppercase") && pass.contains(&c.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_uppercase") && pass.contains(&c.to_string()) {
                ok_uppercase = true;
            }
        }
        if !ok_uppercase && config.cli_args.is_present("no_uppercase") {
            ok_uppercase = true;
        }
        ok_uppercase
    }

    fn validate_lowercase(config: &Configurator, pass: String) -> bool {
        let mut ok_lowercase = false;
        for e in 'a' as u8..'z' as u8 + 1 {
            let c = e as char;
            if config.cli_args.is_present("no_lowercase") && pass.contains(&c.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_lowercase") && pass.contains(&c.to_string()) {
                ok_lowercase = true;
            }
        }
        if !ok_lowercase && config.cli_args.is_present("no_lowercase") {
            ok_lowercase = true;
        }
        ok_lowercase
    }

    fn validate_numbers(config: &Configurator, pass: String) -> bool {
        let mut ok_numbers = false;
        for e in '0' as u8..'9' as u8 + 1 {
            let c = e as char;
            if config.cli_args.is_present("no_numbers") && pass.contains(&c.to_string()) {
                return false;
            } else if !config.cli_args.is_present("no_numbers") && pass.contains(&c.to_string()) {
                ok_numbers = true;
            }
        }
        if !ok_numbers && config.cli_args.is_present("no_numbers") {
            ok_numbers = true;
        }
        ok_numbers
    }
}

#[test]
fn test_pass_generate_works_when_typical() {
    use clap::App;
    let arg_vec = vec!["randompass"];
    let cli_args = App::new("randompass").get_matches_from(arg_vec);

    let actual = Password::generate(&Configurator { cli_args });

    assert!(actual.len() == constants::DEFAULT_PASS_LEN);
}

#[test]
fn test_pass_generate_works_when_no_spcial_characters() {
    use clap::{App, Arg};
    let arg_vec = vec!["randompass", "-c"];
    let cli_args = App::new("randompass")
        .arg(
            Arg::with_name("no_special_chars")
                .short("c")
                .long("no_special_chars"),
        )
        .get_matches_from(arg_vec);
    let config = Configurator { cli_args };

    let actual = Password::generate(&config);

    assert!(actual.len() == constants::DEFAULT_PASS_LEN);
    assert!(Password::validate_special_chars(&config, actual));
}

#[test]
fn test_pass_generate_works_when_no_lowercase() {
    use clap::{App, Arg};
    let arg_vec = vec!["randompass", "-o"];
    let cli_args = App::new("randompass")
        .arg(
            Arg::with_name("no_lowercase")
                .short("o")
                .long("no_lowercase"),
        )
        .get_matches_from(arg_vec);
    let config = Configurator { cli_args };

    let actual = Password::generate(&config);

    assert!(actual.len() == constants::DEFAULT_PASS_LEN);
    assert!(Password::validate_lowercase(&config, actual));
}

#[test]
fn test_pass_generate_works_when_no_uppercase() {
    use clap::{App, Arg};
    let arg_vec = vec!["randompass", "-u"];
    let cli_args = App::new("randompass")
        .arg(
            Arg::with_name("no_uppercase")
                .short("u")
                .long("no_uppercase"),
        )
        .get_matches_from(arg_vec);
    let config = Configurator { cli_args };

    let actual = Password::generate(&config);

    assert!(actual.len() == constants::DEFAULT_PASS_LEN);
    assert!(Password::validate_lowercase(&config, actual));
}

#[test]
fn test_pass_generate_works_when_no_numbers() {
    use clap::{App, Arg};
    let arg_vec = vec!["randompass", "-n"];
    let cli_args = App::new("randompass")
        .arg(Arg::with_name("no_numbers").short("n").long("no_numbers"))
        .get_matches_from(arg_vec);
    let config = Configurator { cli_args };

    let actual = Password::generate(&config);

    assert!(actual.len() == constants::DEFAULT_PASS_LEN);
    assert!(Password::validate_lowercase(&config, actual));
}
