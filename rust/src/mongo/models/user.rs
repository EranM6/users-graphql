use serde::{Deserialize, Serialize, Serializer};
use mongodb::bson;
// use bson::DateTime;
use crate::mongo::MongoError;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

extern crate chrono;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option<bson::oid::ObjectId>,
    pub percent_positive: f64,
    pub recovered :i32,
    pub dead :i32,
    pub severe_condition:i32,
    pub daily_tests:i32,
    pub ventilated :i32,
    #[serde(default)]
    pub hospitalized :i32,
    pub date : String,
    pub infected : i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by : String,
    pub updated_by : String,
}

impl Serialize for DateTime<Utc> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_i64(self.timestamp())
    }
}