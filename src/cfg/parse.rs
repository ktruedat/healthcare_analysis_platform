use crate::cfg::config::Config;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn parse_yaml_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader)?;
    Ok(config)
}
