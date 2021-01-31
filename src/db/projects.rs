use crate::models::project::*;
use crate::schema::project;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "project"]
pub struct NewProject<'a> {
    title: &'a str,
    description: &'a str,
    categories: &'a str,
    creator_id: i32,
}

pub fn create(
    conn: &MysqlConnection,
    title: &str,
    description: &str,
    categories: &str,
    creator_id: i32,
) -> bool {
    let new_project = &NewProject {
        title,
        description,
        categories,
        creator_id,
    };

    diesel::insert_into(project::table)
        .values(new_project)
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn delete(conn: &MysqlConnection, id: i32) -> bool {
    diesel::delete(project::table.filter(project::id.eq(id)))
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

#[derive(Deserialize, AsChangeset, Default)]
#[table_name = "project"]
pub struct UpdateProjectData {
    title: Option<String>,
    description: Option<String>,
    categories: Option<String>,
}

pub fn update(conn: &MysqlConnection, id: i32, data: &UpdateProjectData) -> bool {
    diesel::update(project::table.filter(project::id.eq(id)))
        .set(data)
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn find(conn: &MysqlConnection, id: i32) -> Option<Project> {
    project::table
        .filter(project::id.eq(id))
        .get_result::<Project>(conn)
        .ok()
}
