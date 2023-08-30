use serde_derive::{Serialize, Deserialize};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    version: u8,
    token: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "MONK-E".to_string(),
            version: 0,
            token: "TOKEN".to_string(),
        }
    }
}


fn main() -> Result<(), confy::ConfyError> {
    println!("Hello World!");
    let config: Config = confy::load("MONK-E", None)?;
    let file = confy::get_configuration_file_path("MONK-E", None)?;
    println!("The configuration file path is: {:#?}", file);
    println!("The configuration is:");
    println!("{:#?}", config);
    println!("The wrote toml file content is:");
    let mut content = String::new();
    std::fs::File::open(&file)
        .expect("Failed to open toml configuration file.")
        .read_to_string(&mut content)
        .expect("Failed to read toml configuration file.");
    println!("{}", content);
    Ok(())
}
