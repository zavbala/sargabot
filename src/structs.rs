use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Beach {
    pub name: String,
    pub status: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_public: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}
