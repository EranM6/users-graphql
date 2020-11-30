use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::prelude::*;
use crate::mongo::mongo_config::MongoConfig;
use crate::pubsub::PubSubConfig;
use serde_json::Value;


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigObj {
    pub mongo: MongoConfig,
    pub pubsub: PubSubConfig
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub conf: ConfigObj,
}

impl Config {
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }

    pub fn init<T>(profile: String) {
        let mut file = File::open(format!("{}/config.json", env!("CARGO_MANIFEST_DIR"))).expect("Unable to open file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let contents: Value = serde_json::from_str(&contents).unwrap();
        let x = &contents[&profile];

        let config: ConfigObj = serde_json::from_str(&x.to_string()).unwrap();

        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(Self{ conf: config }))

    }
}

thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}