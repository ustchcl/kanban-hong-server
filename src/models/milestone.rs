use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::schema::milestone;
use crate::models::project::{ProjectJson, Project};

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Project)]
#[table_name="milestone"]
pub struct Milestone {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub deadline: DateTime<Utc>,
    pub project_id: i32,
}

impl Milestone {
    pub fn attach(self, project: ProjectJson) -> MilestoneJson {
        MilestoneJson {
            id: self.id,
            title: self.title,
            description: self.description,
            deadline: self.deadline,
            project,
        }
    }
}

#[derive(Serialize)]
pub struct MilestoneJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub deadline: DateTime<Utc>,
    pub project: ProjectJson,
}
