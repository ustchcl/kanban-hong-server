use serde::Serialize;
use crate::schema::task;
use crate::models::{account::Account, milestone::Milestone};

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "task"]
pub struct Task {
    id: i32,
    title: String,
    description: String,
    parent_id: Option<i32>,
    category: String,
    creator_id: i32,
    milestone_id: Option<i32>,
}

impl Task {
    pub fn attach(self) {
    }
}

#[derive(Serialize)]
pub struct TaskJson<'a> {
    id: i32,
    title: &'a str,
    description: &'a str,
    tasks: Vec<&'a TaskJson<'a>>,
    creator: &'a Account,
    milestone: Option<&'a Milestone>,
}
