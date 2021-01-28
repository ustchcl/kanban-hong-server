use crate::models::task::Task;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::Deserialize;
use crate::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
   title: &'a str,
   description: &'a str,
   category: &'a str,
   creator_id: i32,
   parent_id: Option<i32>,
   milestone_id: Option<i32>,
}

pub fn create(
    conn: &MysqlConnection,
    title: &str,
    description: &str,
    category: &str,
    creator_id: i32,
    parent_id: Option<i32>,
    milestone_id: Option<i32>,
) -> bool {
    let new_task = &NewTask {
        title,
        description,
        category,
        creator_id,
        parent_id,
        milestone_id,
    };

    diesel::insert_into(task::table)
        .values(new_task)
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn delete(
    conn: &MysqlConnection,
    id: i32
) -> bool {
    diesel::delete(task::table.filter(task::id.eq(id)))
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

#[derive(Deserialize, AsChangeset, Default)]
#[table_name = "task"]
pub struct UpdateTaskData {
    title: Option<String>,
    description: Option<String>,
    category: Option<String>,
    parent_id: Option<i32>,
    milestone_id: Option<i32>,    
}

pub fn update(
    conn: &MysqlConnection,
    id: i32,
    data: &UpdateProjectData
) -> bool {
    diesel::update(
        task::table.filter(task::id.eq(id))
    ).set(data)
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn find(
    conn: &MysqlConnection,
    id: i32
) -> Option<Task> {
    task::table.filter(task::id.eq(id))
        .get_result::<Task>()
        .ok()
}

