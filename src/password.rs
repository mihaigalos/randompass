use crate::alphabet::Alphabet;
use crate::config::Configurator;
use crate::constants;
use crate::error::ValidateError;

pub struct Password {}

impl Password {
    pub fn generate(config: &Configurator, mut alphabet: Alphabet) -> String {
        let mut watchdog: usize = constants::MAX_CONVERGENCE_ITERATIONS;
        loop {
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

            if Password::validate(&config, &pass).is_ok() {
                return pass;
            } else {
                if watchdog == 0 {
                    break;
                }
            }
            watchdog = watchdog - 1;
        }
        return "".to_string();
    }

    fn validate(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        Password::validate_length(&pass)?;
        Password::validate_special_chars(config, &pass)?;
        Password::validate_uppercase(config, &pass)?;
        Password::validate_lowercase(config, &pass)?;
        Password::validate_numbers(config, &pass)?;
        Ok(())
    }

    fn validate_length(pass: &str) -> Result<(), ValidateError> {
        let length = pass.len();
        match length {
            0 => Err(ValidateError::InvalidLength),
            _ => Ok(()),
        }
    }
    fn validate_special_chars(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in constants::SPECIAL_CHARS.to_vec().iter() {
            if config.cli_args.is_present("no_special_chars") && pass.contains(&e.to_string()) {
                return Err(ValidateError::NoSpecialChars);
            } else if !config.cli_args.is_present("no_special_chars")
                && pass.contains(&e.to_string())
            {
                return Ok(());
            }
        }
        if config.cli_args.is_present("no_special_chars") {
            return Ok(());
        }
        Err(ValidateError::NoSpecialChars)
    }

    fn validate_uppercase(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in 'A' as u8..'Z' as u8 + 1 {
            let c = e as char;
            if config.cli_args.is_present("no_uppercase") && pass.contains(&c.to_string()) {
                return Err(ValidateError::NoUpperCase);
            } else if !config.cli_args.is_present("no_uppercase") && pass.contains(&c.to_string()) {
                return Ok(());
            }
        }
        if config.cli_args.is_present("no_uppercase") {
            return Ok(());
        }
        Err(ValidateError::NoUpperCase)
    }
    fn validate_lowercase(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in 'a' as u8..'z' as u8 + 1 {
            let c = e as char;
            if config.cli_args.is_present("no_lowercase") && pass.contains(&c.to_string()) {
                return Err(ValidateError::NoLowerCase);
            } else if !config.cli_args.is_present("no_lowercase") && pass.contains(&c.to_string()) {
                return Ok(());
            }
        }
        if config.cli_args.is_present("no_lowercase") {
            return Ok(());
        }
        Err(ValidateError::NoLowerCase)
    }
    fn validate_numbers(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in '0' as u8..'9' as u8 + 1 {
            let c = e as char;
            if config.cli_args.is_present("no_numbers") && pass.contains(&c.to_string()) {
                return Err(ValidateError::NoNumbers);
            } else if !config.cli_args.is_present("no_numbers") && pass.contains(&c.to_string()) {
                return Ok(());
            }
        }
        if config.cli_args.is_present("no_numbers") {
            return Ok(());
        }
        Err(ValidateError::NoNumbers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    #[test]
    fn test_pass_generate_works_when_typical() {
        use clap::App;
        let arg_vec = vec!["randompass"];
        let cli_args = App::new("randompass").get_matches_from(arg_vec);
        let config = Configurator { cli_args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        println!("len: {}", actual.len());
        assert!(actual.len() == constants::DEFAULT_PASS_LEN);
    }

    #[test]
    fn test_pass_generate_works_when_no_special_characters() {
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

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_special_chars(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_special_characters_but_none_requested() {
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

        let actual = "#$!".to_string();

        assert_err!(
            Password::validate_special_chars(&config, &actual),
            Err(ValidateError::NoSpecialChars)
        );
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

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_lowercase(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_lowercase_but_none_requested() {
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

        let actual = "abc".to_string();

        assert_err!(
            Password::validate_lowercase(&config, &actual),
            Err(ValidateError::NoLowerCase)
        );
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

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_uppercase(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_uppercase_but_none_requested() {
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

        let actual = "ABC".to_string();

        assert_err!(
            Password::validate_uppercase(&config, &actual),
            Err(ValidateError::NoUpperCase)
        );
    }

    #[test]
    fn test_pass_generate_works_when_no_numbers() {
        use clap::{App, Arg};
        let arg_vec = vec!["randompass", "-n"];
        let cli_args = App::new("randompass")
            .arg(Arg::with_name("no_numbers").short("n").long("no_numbers"))
            .get_matches_from(arg_vec);
        let config = Configurator { cli_args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_numbers(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_numbers_but_none_requested() {
        use clap::{App, Arg};
        let arg_vec = vec!["randompass", "-n"];
        let cli_args = App::new("randompass")
            .arg(Arg::with_name("no_numbers").short("n").long("no_numbers"))
            .get_matches_from(arg_vec);
        let config = Configurator { cli_args };

        let actual = "123".to_string();

        assert_err!(
            Password::validate_numbers(&config, &actual),
            Err(ValidateError::NoNumbers)
        );
    }

    #[test]
    fn test_pass_generate_fails_when_impossible_constraints() {
        use clap::{App, Arg};
        use rand::distributions::Uniform;
        use rand::rngs::StdRng;
        use rand::SeedableRng;

        let arg_vec = vec!["randompass", "-c", "-u", "-o", "-n"];
        let cli_args = App::new("randompass")
            .arg(
                Arg::with_name("no_special_chars")
                    .short("c")
                    .long("no_special_chars"),
            )
            .arg(
                Arg::with_name("no_uppercase")
                    .short("u")
                    .long("no_uppercase"),
            )
            .arg(
                Arg::with_name("no_lowercase")
                    .short("o")
                    .long("no_lowercase"),
            )
            .arg(Arg::with_name("no_numbers").short("n").long("no_numbers"))
            .get_matches_from(arg_vec);
        let config = Configurator { cli_args };
        let chars: Vec<char> = vec!['a'];
        let alphabet = Alphabet {
            chars,
            range: Uniform::new(0, 1),
            rng: StdRng::from_seed([0u8; 32]),
        };

        let actual = Password::generate(&config, alphabet);

        println!("{}", actual);

        assert_err!(
            Password::validate_length(&actual),
            Err(ValidateError::InvalidLength)
        );
    }
}
