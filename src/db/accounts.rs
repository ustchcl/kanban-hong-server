use std::usize;

use crate::models::account::Account;
use crate::schema::account;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub icon: Option<&'a str>,
}

pub struct DuplicatedUsername;

impl From<Error> for DuplicatedUsername {
    fn from(err: Error) -> DuplicatedUsername {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("accout_username_key") => return DuplicatedUsername,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &MysqlConnection,
    username: &str,
    password: &str,
    icon: Option<&str>,
) -> Result<usize, DuplicatedUsername> {
    let new_account = &NewAccount {
        username,
        password,
        icon,
    };

    diesel::insert_into(account::table)
        .values(new_account)
        .execute(conn)
        .map_err(Into::into)
}

pub fn login(conn: &MysqlConnection, username: &str, password: &str) -> Option<Account> {
    let account = account::table
        .filter(account::username.eq(username))
        .filter(account::password.eq(password))
        .get_result::<Account>(conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    Some(account)
}

pub fn find(conn: &MysqlConnection, id: i32) -> Option<Account> {
    account::table
        .filter(account::id.eq(id))
        .get_result::<Account>(conn)
        .map_err(|err| eprintln!("get_account_by_id: {}", err))
        .ok()
}

#[derive(Deserialize, AsChangeset, Default)]
#[table_name = "account"]
pub struct UpdateAccountData {
    username: Option<String>,
    password: Option<String>,
    icon: Option<String>,
}

pub fn update(conn: &MysqlConnection, id: i32, data: &UpdateAccountData) -> Result<usize, DuplicatedUsername> {
    diesel::update(account::table.find(id))
        .set(data)
        .execute(conn)
        .map_err(Into::into)
}


