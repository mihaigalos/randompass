use autoclap::autoclap;
use clap::{Arg, ArgAction, Command};
use randompass::alphabet::Alphabet;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app = autoclap!();
    let args = app
        .arg(
            Arg::new("length")
                .long("length")
                .short('l')
                .help("Password length.")
                .required(false),
        )
        .arg(
            Arg::new("no_lowercase")
                .long("no_lowercase")
                .short('o')
                .action(ArgAction::SetTrue)
                .help("Disable usage of lowercase letters.")
                .required(false),
        )
        .arg(
            Arg::new("no_numbers")
                .long("no_numbers")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Disable usage of numbers.")
                .required(false),
        )
        .arg(
            Arg::new("no_special_chars")
                .long("no_special_chars")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Disable usage of special characters (i.e.: !, $, #).")
                .required(false),
        )
        .arg(
            Arg::new("no_uppercase")
                .long("no_uppercase")
                .short('u')
                .action(ArgAction::SetTrue)
                .help("Disable usage of uppercase letters.")
                .required(false),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    let config = randompass::config::Configurator { args };
    let pass = randompass::password::Password::generate(&config, Alphabet::new(&config));
    if !pass.is_empty(){
        println!("{}", pass);
    } else {
        println!(
            "ERROR: Cannot generate password after MAX iterations. Consider lowering constraints."
        );
    }
}
