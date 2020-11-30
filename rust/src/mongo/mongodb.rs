use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection, bson};
use crate::configuration::Config;
use std::env::VarError;
use super::MongoError;
use super::models::User;
use std::sync::Arc;
use std::rc::Rc;
use mongodb::error::Error;

#[derive(Clone, Debug)]
pub struct MongoDb {
    client: Client,
    db_name: String,
}

impl MongoDb {
    pub async fn init() -> Result<Self, MongoError> {
        // let mut client_options = ClientOptions::parse(&get_conn_str()?).await?;
        let mut client_options = ClientOptions::parse("mongodb://eranm:eranm6@localhost:27017").await?;
        // dbg!(&client_options);
        client_options.app_name = Some("users".to_string());
        // let client = Client::with_options(client_options)?;

        let client = mongodb::Client::with_uri_str(&get_conn_str()?).await?;
        for name in client.list_database_names(None, None).await? {
            println!("- {}", name);
        }

        Ok(Self {
            client,
            db_name: String::from(&Config::current().conf.mongo.database)
        })
    }

    pub async fn user(&self) -> Result<User, MongoError> {
        let courser = self
            .get_collection()
            .find_one(None, None)
            .await?
            .expect("user not found");

        println!("{}", courser);

        let result: User = bson::from_bson(Bson::Document(courser))?;

        Ok(result)
    }

    fn get_collection(&self) -> Collection {
        self.client.database(&self.db_name).collection("test")
    }
}

fn get_conn_str() -> Result<String, VarError> {
    let config = Config::current();
    config.conf.mongo.conn_url()
}