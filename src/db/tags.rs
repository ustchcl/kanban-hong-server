use diesel::prelude::*;
use crate::schema::tag;
use crate::models::tag::Tag;
use diesel::mysql::MysqlConnection;

#[derive(Insertable)]
#[table_name = "tag"]
pub struct NewTag<'a> {
    name: &'a str,
}

pub fn create(
    conn: &MysqlConnection,
    name: &str
) -> bool {
    let new_tag = &NewTag {
        name,
    };

    diesel::insert_into(tag::table)
        .values(new_tag)
        .execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn delete(
    conn: &MysqlConnection,
    id: i32
) -> bool {
    diesel::delete(
        tag::table.filter(tag::id.eq(id))
    ).execute(conn)
        .ok()
        .map_or(false, |affected_rows| affected_rows > 0)
}

pub fn find(
    conn: &MysqlConnection,
    id: i32
) -> Option<Tag> {
    tag::table.filter(
        tag::id.eq(id)
    ).get_result::<Tag>(conn)
        .ok()
}
