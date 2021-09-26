#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index])
        .attach(db::Conn::fairing())
}
