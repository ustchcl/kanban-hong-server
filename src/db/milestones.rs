use chrono::NaiveDateTime;
use crate::schema::milestone;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "milestone"]
pub struct NewMilestone<'a> {
  pub title: &'a str,
  pub description: &'a str,
  pub deadline: &'a NaiveDateTime,
  pub project_id: i32,
}

pub fn create(
    conn: &MysqlConnection,
    title: &str,
    description: &str,
    deadline: &NaiveDateTime,
    project_id: i32
) -> Option<bool> {
    let new_milestone = &NewMilestone {
        title,
        description,
        deadline,
        project_id
    };
    diesel::insert_into(milestone::table)
        .values(new_milestone)
        .execute(conn)
        .map(|affected_rows| affected_rows > 0)
        .ok()
}

#[derive(Deserialize, AsChangeset, Default)]
#[table_name = "milestone"]
pub struct UpdateMilestoneData {
    title: Option<String>,
    description: Option<String>,
    deadline: Option<NaiveDateTime>,
    project_id: Option<i32>,
}

pub fn update(
    conn: &MysqlConnection,
    data: &UpdateMilestoneData
) -> Option<bool> {
    Some(true)
}

    
