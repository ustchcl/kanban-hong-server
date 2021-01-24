use chrono::{DateTime, Utc};
use serde::Serialize;
use diesel::deserialize;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub icon: String,
    pub create_time: DateTime<Utc>,
}

impl Account {
    
}



