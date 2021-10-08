#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use rocket::figment::util::map;
use rocket::Config;
use std::env;
use std::fs;

mod db;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "integer".to_string());
    let db_user = env::var("DB_USER").unwrap();
    let db_password_file = env::var("DB_PASSWORD_FILE").unwrap();
    let db_password = fs::read_to_string(db_password_file)
        .unwrap()
        .trim()
        .to_string();
    let db_uri = format!(
        "postgresql://{}:{}@{}/{}",
        db_user, db_password, db_host, db_name
    );
    println!("DB URI - {}", db_uri);

    let figment = Config::figment().merge(("databases", map!["integer" => map!["url" => db_uri]]));

    rocket::custom(figment)
        .mount("/", routes![routes::index])
        .mount(
            "/members",
            routes![routes::members::find, routes::members::find_contact,],
        )
        .attach(db::Conn::fairing())
}
