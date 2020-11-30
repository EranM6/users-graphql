mod pubsub;
mod mongo;
mod configuration;

extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate bson;

use tokio;
use std::env;
use configuration::Config;
use mongo::MongoConfig;
use pubsub::PubSubConfig;
use crate::mongo::mongodb::MongoDb;
use std::error::Error;
use crate::mongo::MongoError;

#[derive(Default, Debug, Serialize, Deserialize)]
struct ConfigObj {
    pub mongo: MongoConfig,
    pub pubsub: PubSubConfig
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let profile = env::var("PROFILE").unwrap_or(String::from("stage"));
    Config::init::<ConfigObj>(profile);

        let mongo = MongoDb::init().await;
        match mongo {
            Ok(client) => {
                let user = client.user().await;
                match user {
                    Ok(user) => {
                        println!("{:?}", user)
                    }
                    Err(e) => println!("{}", e.message())
                }
            }
            Err(e) => println!("{}", e.message())
        }

    println!("profile = {:?}", Config::current().conf.mongo.conn_url());
Ok(())
}
