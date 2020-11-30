use std::env;
use std::sync::Arc;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct MongoConfig {
    pub hosts: Vec<String>,
    pub port: String,
    pub database: String,
    #[serde(default)]
    pub admin_db: String,
    #[serde(default)]
    pub rs_name: String,
}

impl MongoConfig {
    pub fn db_name(&self) -> &str { &self.database }
    pub fn conn_url(&self) -> Result<String, std::env::VarError> {
        let ips: Vec<String> = Arc::new(&self.hosts).into_iter().map(|ip| format!("{}:{}", &ip, self.port)).collect();
        let mongo_user = env::var("MONGO_USER")?;
        let mongo_pass = env::var("MONGO_PASS")?;
        let rs_str = if self.rs_name.len() > 0 {
            format!("&replicaSet={}", self.rs_name)
        } else { Default::default() };

        let auth_source = if self.admin_db.len() > 0 {
            format!("?authSource={}", self.admin_db)
        } else { Default::default() };

        Ok(format!(
            "mongodb://{}:{}@{}/{}{}{}",
            mongo_user,
            mongo_pass,
            ips.join(","),
            self.database,
            auth_source,
            rs_str
        ))
    }
}