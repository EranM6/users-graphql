use crate::models::User;
use crate::mongo::mongodb::MongoDb;
use async_graphql::Object;
pub struct Query {
    pub mongo_client: MongoDb
}

#[Object(extends)]
impl Query {
    async fn user(&self) -> Option<User> {
        self.mongo_client.random_user().await.ok().unwrap()
    }
}
