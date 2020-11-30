use super::{MongoError, MongoConfig};
use crate::models::User;
use mongodb::bson::{doc, Bson};
use mongodb::{Database, Collection};
use mongodb::{bson, options::ClientOptions, Client};
use tokio_stream::StreamExt;

#[derive(Clone, Debug)]
pub struct MongoDb {
    client: Client,
    db_name: String
}

impl MongoDb {
    pub async fn init(config: &MongoConfig) -> Result<Self, MongoError> {
        let client_options = ClientOptions::parse(config.conn_url()?).await?;
        let client = Client::with_options(client_options)?;

        Ok(Self {
            client,
            db_name: String::from(config.db_name())
        })
    }
    async fn get_db(&self) -> Result<Database, MongoError> {
        Ok(self.client.database(&self.db_name))
    }

    pub async fn random_user(&self) -> Result<Option<User>, MongoError> {
        let pipeline = vec![doc! {
          "$sample": {
              "size": Bson::Int32(1)
            }
        }];

        let coll: Collection<User> = Self::get_db(self)
            .await
            .ok()
            .unwrap().collection("users");

        let mut results = coll.aggregate(pipeline, None)
            .await?;

        let mut users = vec![];

        while let Some(doc) = results.next().await {
            let user = bson::from_bson::<User>(Bson::Document(doc?))?;
            users.push(user);
        }

        Ok(users.get(0).cloned())
    }
}
