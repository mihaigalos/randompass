use autoclap::autoclap;
use clap::{App, Arg};
use randompass::alphabet::Alphabet;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app = autoclap!();
    let args = app
        .arg(
            Arg::new("length")
                .long("length")
                .short('l')
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
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    let config = randompass::config::Configurator { args };
    let pass = randompass::password::Password::generate(&config, Alphabet::new(&config));
    if pass.len() > 0 {
        println!("{}", pass);
    } else {
        println!(
            "ERROR: Cannot generate password after MAX iterations. Consider lowering constraints."
        );
    }
}
