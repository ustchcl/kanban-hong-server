#![allow(unused_imports)]
#![allow(dead_code)]
extern crate dotenv;
#[macro_use]
extern crate diesel;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use dotenv::dotenv;
use std::env;
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{Pool, ConnectionManager},
};
mod schema;
mod config;
mod models;
mod db;
mod routes;
mod errors;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
        "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(config::SECRET.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain("49.234.74.97")
                    .secure(false), // 只有使用https时才可开启
                    ))
            .wrap(middleware::DefaultHeaders::new()
                  .header("Access-Control-Allow-Origin", "http://localhost:8080")
                  .header("Access-Control-Allow-Credentials", "true")
                  .header("Access-Control-Allow-Headers", "Authorization,Content-Type,Accept,Origin,User-Agent,DNT,Cache-Control,X-Mx-ReqToken,X-Requested-With")
                  .header("Access-Control-Allow-Methods", "GET,POST,OPTIONS,DELETE,PUT,HEAD")
                 )
            .route("/", web::get().to(index))
    })
    .workers(2)
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
