mod pubsub;
mod mongo;
mod configuration;

extern crate serde;
extern crate serde_json;
extern crate bson;

#[macro_use]
extern crate serde_derive;

use tokio;
use std::env;
use configuration::Config;
use crate::mongo::mongodb::MongoDb;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let profile = env::var("PROFILE").unwrap_or(String::from("stage"));

    Config::init(profile);
    let mongo = MongoDb::init().await;
    match mongo {
        Ok(client) => {
            let user = client.user().await;
            match user {
                Ok(user) => {
                    println!("{}", serde_json::to_string(&user).ok().unwrap());
                    println!("{}", user.register_date.to_rfc3339());
                    println!("{}", user.update_date.to_rfc3339())
                }
                Err(e) => println!("{}", e.message())
            }
        }
        Err(e) => println!("{}", e.message())
    }

    Ok(())
}
