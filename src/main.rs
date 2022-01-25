use clap::crate_version;
use clap::{App, Arg};
use randompass::alphabet::Alphabet;

#[cfg(not(tarpaulin_include))]
fn main() {
    let cli_args = App::new(concat!(
        env!("CARGO_CRATE_NAME"),
        " ",
        env!("CARGO_PKG_VERSION"),
        " :: ",
        concat!(
            env!("CARGO_PKG_REPOSITORY"),
            "/releases/tag/",
            crate_version!()
        )
    ))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(
        Arg::with_name("length")
            .long("length")
            .short("l")
            .help("Password length."),
    )
    .arg(
        Arg::with_name("no_lowercase")
            .long("no_lowercase")
            .short("o")
            .help("Disable usage of lowercase letters."),
    )
    .arg(
        Arg::with_name("no_numbers")
            .long("no_numbers")
            .short("n")
            .help("Disable usage of numbers."),
    )
    .arg(
        Arg::with_name("no_special_chars")
            .long("no_special_chars")
            .short("c")
            .help("Disable usage of special characters (i.e.: !, $, #)."),
    )
    .arg(
        Arg::with_name("no_uppercase")
            .long("no_uppercase")
            .short("u")
            .help("Disable usage of uppercase letters."),
    )
    .get_matches_safe()
    .unwrap_or_else(|e| e.exit());

    let config = randompass::config::Configurator { cli_args };
    let pass = randompass::password::Password::generate(&config, Alphabet::new(&config));
    if pass.len() > 0 {
        println!("{}", pass);
    } else {
        println!(
            "ERROR: Cannot generate password after MAX iterations. Consider lowering constraints."
        );
    }
}
