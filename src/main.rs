use clap::{clap_app, crate_version};

fn main() {
    let cli_args = clap_app!(randompass =>
        (version: crate_version!())
        (author: "Mihai Galos <mihaigalos at gmail dot com>")
            (@arg length:           -l --length              "Disable usage of lowercase letters.")
            (@arg no_lowercase:     -w --no_lowercase        "Disable usage of lowercase letters.")
            (@arg no_numbers:       -n --no_numbers          "Disable usage of numbers.")
            (@arg no_special_chars: -c --no_special_chars    "Disable usage of special characters (i.e.: !, $, #).")
            (@arg no_uppercase:     -u --no_uppercase        "Disable usage of uppercase letters.")

    )
    .get_matches_safe()
    .unwrap_or_else(|e| e.exit());

    let config = randompass::config::Configurator { cli_args };
    let mut alphabet = randompass::alphabet::Alphabet::new(&config);
    let length: usize = config
        .cli_args
        .value_of("length")
        .unwrap_or(&randompass::constants::DEFAULT_PASS_LEN.to_string())
        .parse()
        .unwrap();

    let mut pass = String::new();
    for _ in 0..length {
        pass.push(alphabet.get_char());
    }

    println!("{}", pass);
}
