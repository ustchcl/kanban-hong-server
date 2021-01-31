use crate::config::DATE_FORMAT;
use crate::models::account::Account;
use serde::Serialize;
use chrono::NaiveDateTime;
use crate::schema::project;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Account, foreign_key = "creator_id")]
#[table_name="project"]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub categories: String,
    pub creator_id: i32,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl Project {
    pub fn attach(self, creator: Account) -> ProjectJson {
        ProjectJson {
            id: self.id,
            title: self.title,
            description: self.description,
            categories: self.categories,
            create_time: self.create_time,
            creator,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub categories: String,
    pub create_time: NaiveDateTime,
    pub creator: Account,
}
