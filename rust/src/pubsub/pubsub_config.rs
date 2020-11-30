use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PubSubConfig {
    project_id: String,
    subscriptions: HashMap<String, String>,
}