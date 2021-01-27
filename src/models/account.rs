use chrono::{DateTime, Utc, NaiveDateTime};
use serde::Serialize;
use diesel::deserialize;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub icon: Option<String>,
    pub create_time: NaiveDateTime,
}

impl Account {
    
}



