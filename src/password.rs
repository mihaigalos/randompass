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
                .args
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
            if config.args.is_present("no_special_chars") && pass.contains(&e.to_string()) {
                return Err(ValidateError::NoSpecialChars);
            } else if !config.args.is_present("no_special_chars") && pass.contains(&e.to_string()) {
                return Ok(());
            }
        }
        Ok(())
    }

    fn validate_uppercase(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in 'A' as u8..'Z' as u8 + 1 {
            let c = e as char;
            if config.args.is_present("no_uppercase") && pass.contains(&c.to_string()) {
                return Err(ValidateError::NoUpperCase);
            } else if !config.args.is_present("no_uppercase") && pass.contains(&c.to_string()) {
                return Ok(());
            }
        }
        Ok(())
    }
    fn validate_lowercase(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in 'a' as u8..'z' as u8 + 1 {
            let c = e as char;
            if config.args.is_present("no_lowercase") && pass.contains(&c.to_string()) {
                return Err(ValidateError::NoLowerCase);
            } else if !config.args.is_present("no_lowercase") && pass.contains(&c.to_string()) {
                return Ok(());
            }
        }
        Ok(())
    }
    fn validate_numbers(config: &Configurator, pass: &str) -> Result<(), ValidateError> {
        for e in '0' as u8..'9' as u8 + 1 {
            let c = e as char;
            if config.args.is_present("no_numbers") && pass.contains(&c.to_string()) {
                return Err(ValidateError::NoNumbers);
            } else if !config.args.is_present("no_numbers") && pass.contains(&c.to_string()) {
                return Ok(());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matches_from(arg_vec: Vec<&str>) -> clap::ArgMatches {
        use clap::{App, Arg};
        let args = App::new("randompass")
            .arg(
                Arg::new("length")
                    .long("length")
                    .short('l')
                    .takes_value(true)
                    .help("Password length."),
            )
            .arg(
                Arg::new("no_lowercase")
                    .long("no_lowercase")
                    .short('o')
                    .help("Disable usage of lowercase letters."),
            )
            .arg(
                Arg::new("no_numbers")
                    .long("no_numbers")
                    .short('n')
                    .help("Disable usage of numbers."),
            )
            .arg(
                Arg::new("no_special_chars")
                    .long("no_special_chars")
                    .short('c')
                    .help("Disable usage of special characters (i.e.: !, $, #)."),
            )
            .arg(
                Arg::new("no_uppercase")
                    .long("no_uppercase")
                    .short('u')
                    .help("Disable usage of uppercase letters."),
            )
            .get_matches_from(arg_vec);
        args
    }

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
        let args = matches_from(vec!["randompass"]);
        let config = Configurator { args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        println!("len: {}", actual.len());
        assert!(actual.len() == constants::DEFAULT_PASS_LEN);
    }

    #[test]
    fn test_pass_generate_works_when_length_requested() {
        let required_length = 32;
        let args = matches_from(vec!["randompass", "-l", &required_length.to_string()]);
        let config = Configurator { args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(actual.len() == required_length);
    }

    #[test]
    fn test_pass_generate_works_when_no_special_characters() {
        let args = matches_from(vec!["randompass", "-c"]);
        let config = Configurator { args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_special_chars(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_special_characters_but_none_requested() {
        let args = matches_from(vec!["randompass", "-c"]);
        let config = Configurator { args };

        let actual = "#$!".to_string();

        assert_err!(
            Password::validate_special_chars(&config, &actual),
            Err(ValidateError::NoSpecialChars)
        );
    }

    #[test]
    fn test_pass_generate_works_when_no_lowercase() {
        let args = matches_from(vec!["randompass", "-o"]);
        let config = Configurator { args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_lowercase(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_lowercase_but_none_requested() {
        let args = matches_from(vec!["randompass", "-o"]);
        let config = Configurator { args };

        let actual = "abc".to_string();

        assert_err!(
            Password::validate_lowercase(&config, &actual),
            Err(ValidateError::NoLowerCase)
        );
    }

    #[test]
    fn test_pass_generate_works_when_no_uppercase() {
        let args = matches_from(vec!["randompass", "-u"]);
        let config = Configurator { args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_uppercase(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_uppercase_but_none_requested() {
        let args = matches_from(vec!["randompass", "-u"]);
        let config = Configurator { args };

        let actual = "ABC".to_string();

        assert_err!(
            Password::validate_uppercase(&config, &actual),
            Err(ValidateError::NoUpperCase)
        );
    }

    #[test]
    fn test_pass_generate_works_when_no_numbers() {
        let args = matches_from(vec!["randompass", "-n"]);
        let config = Configurator { args };

        let actual = Password::generate(&config, Alphabet::new(&config));

        assert!(Password::validate_numbers(&config, &actual).is_ok());
    }

    #[test]
    fn test_pass_generate_fails_when_numbers_but_none_requested() {
        let args = matches_from(vec!["randompass", "-n"]);
        let config = Configurator { args };

        let actual = "123".to_string();

        assert_err!(
            Password::validate_numbers(&config, &actual),
            Err(ValidateError::NoNumbers)
        );
    }

    #[test]
    fn test_pass_generate_fails_when_impossible_constraints() {
        use rand::distributions::Uniform;
        use rand::rngs::StdRng;
        use rand::SeedableRng;

        let args = matches_from(vec!["randompass", "-c", "-u", "-o", "-n"]);
        let config = Configurator { args };
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
