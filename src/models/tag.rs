use serde::Serialize;
use crate::schema::tag;

#[derive(Queryable, Serialize, Identifiable)]
#[table_name="tag"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

