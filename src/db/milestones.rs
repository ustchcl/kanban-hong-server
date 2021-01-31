use crate::models::milestone::Milestone;
use crate::schema::milestone;
use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
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
    project_id: i32,
) -> Option<bool> {
    let new_milestone = &NewMilestone {
        title,
        description,
        deadline,
        project_id,
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

pub fn update(conn: &MysqlConnection, id: i32, data: &UpdateMilestoneData) -> bool {
    diesel::update(milestone::table)
        .set(data)
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn delete(conn: &MysqlConnection, id: i32) -> bool {
    diesel::delete(milestone::table.filter(milestone::id.eq(id)))
        .execute(conn)
        .ok()
        .map_or(false, |x| x > 0)
}

pub fn find(conn: &MysqlConnection, id: i32) -> Option<Milestone> {
    milestone::table
        .filter(milestone::id.eq(id))
        .get_result::<Milestone>(conn)
        .ok()
}
