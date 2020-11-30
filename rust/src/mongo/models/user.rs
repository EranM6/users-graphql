use mongodb::bson;
use bson::DateTime;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Debug;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct User {
    // When the document id is of type ObjectId
    // pub id : Option<bson::oid::ObjectId>,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_id: Option<i32>,
    pub password: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub user_mail: String,
    #[serde(default = "User::default_date", serialize_with = "User::format_date")]
    pub update_date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_expiration: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_abuse_token: Option<String>,
    #[serde(rename = "isAbuser", default)]
    pub is_abuser: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<MobilePhone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_brand: Option<Brand>,
    #[serde(serialize_with = "User::format_date")]
    pub register_date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_origin: Option<String>,
    #[serde(default)]
    pub mini_reg_status: bool,
    #[serde(default)]
    pub is_phone_email_conn: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_validation_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_validation_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_agreement: Option<TermsAgreement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<Vec<GDPR>>,
    #[serde(default)]
    pub is_advisor: bool,
    #[serde(default)]
    pub is_paying: bool,
}

impl User {
    fn default_date() -> DateTime {
        DateTime(Utc::now())
    }
    fn format_date<S>(date: &DateTime, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        s.serialize_str(&date.to_rfc3339())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobilePhone {
    phone_prefix: Option<i32>,
    phone_suffix: Option<i32>,
    user_mobile: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Brand { HTZ, TM, HDC }

#[derive(Serialize, Deserialize, Debug)]
pub struct TermsAgreement {
    #[serde(rename = "HTZ")]
    htz: Option<DateTime>,
    #[serde(rename = "TM")]
    tm: Option<DateTime>,
    #[serde(rename = "HDC")]
    hdc: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GDPR {
    brand: Option<String>,
    #[serde(rename = "cookieConsentType")]
    cookie_consent_type: Option<String>,
    #[serde(rename = "cookieConsentDate")]
    cookie_consent_date: Option<DateTime>,
}