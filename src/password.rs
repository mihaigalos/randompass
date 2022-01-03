use crate::alphabet::Alphabet;
use crate::config::Configurator;
use crate::constants;

pub struct Password {}

impl Password {
    pub fn generate(config: Configurator) -> String {
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

        pass
    }
}
