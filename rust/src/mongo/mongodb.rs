use mongodb::bson::{doc, Bson};
use mongodb::{options::ClientOptions, Client, bson};
use crate::configuration::Config;
use std::env::VarError;
use super::MongoError;
use tokio::stream::StreamExt;
use crate::mongo::models::User;

#[derive(Clone, Debug)]
pub struct MongoDb {
    client: Client,
    db_name: String,
}

impl MongoDb {
    pub async fn init() -> Result<Self, MongoError> {
        #[cfg(debug_assertions)]
        println!("{}", &get_conn_str()?);
        let client_options = ClientOptions::parse(&get_conn_str()?).await?;
        let client = Client::with_options(client_options)?;

        Ok(Self {
            client,
            db_name: String::from(Config::current().mongo.db_name()),
        })
    }

    pub async fn user(&self) -> Result<User, MongoError> {
        // Multiple results (::find)
        let mut courser = self
            .client.database(&self.db_name).collection("users")
            .find(Some(doc! {
                "userMail": "amitzur@actcom.co.il"
            }), None)
            .await?;

        while let Some(doc) = courser.next().await {
            println!("{:?}", bson::from_bson::<User>(Bson::Document(doc?))?)
        }

        // Single result (::find_one)
        let doc = self
            .client.database(&self.db_name).collection("users")
            .find_one(Some(doc! {}), None)
            .await?
            .ok_or(MongoError::NotFound);

        let result = bson::from_bson(Bson::Document(doc?))?;

        Ok(result)
    }
}

fn get_conn_str() -> Result<String, VarError> {
    let config = Config::current();
    config.mongo.conn_url()
}