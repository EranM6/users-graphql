use std::env;
use std::sync::Arc;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MongoConfig {
    pub hosts: Vec<String>,
    pub port: String,
    pub database: String,
    pub admin_db: String,
    // pub rs_name: String,
}

impl MongoConfig {
    pub fn db_name(&self) -> &str {&self.database}
    pub fn conn_url(&self) -> Result<String, std::env::VarError> {
        let ips: Vec<String> = Arc::new(&self.hosts).into_iter().map(|ip| format!("{}:{}", &ip, self.port)).collect();
        let mongo_user = env::var("MONGO_USER")?;
        let mongo_pass = env::var("MONGO_PASS")?;
        Ok(format!(
            "mongodb://{}:{}@{}/{}?authSource={}",
            mongo_user,
            mongo_pass,
            ips.join(","),
            self.database,
            self.admin_db
        ))
    }
}