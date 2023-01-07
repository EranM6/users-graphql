mod hierarchy;

use std::fs::File;
use crate::mongo::mongo_config::MongoConfig;
use std::io::{Cursor, Read};
use std::env;
use tomlenv::{Environments, Error};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub mongo: MongoConfig,
}

impl Config {
    pub fn init() -> Result<Self, Error> {
        let mut file = File::open(format!("{}/config.toml", env!("CARGO_MANIFEST_DIR"))).expect("Unable to open file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let mut cursor = Cursor::new(contents);
        let envs: Environments<hierarchy::Hierarchy, Self> = Environments::from_reader(&mut cursor)?;
        let profile = env::var("PROFILE").unwrap_or(String::from("dev"));
        env::set_var("env", profile);
        let current = envs.current()?;
        Ok(current.clone())
    }
}