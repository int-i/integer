#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use rocket::figment::util::map;
use rocket::Config;

mod db;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    let db_config = db::Config::default();
    let figment =
        Config::figment().merge(("databases", map!["integer" => map!["url" => db_config.uri]]));
    rocket::custom(figment)
        .mount(
            "/members",
            routes![routes::members::find, routes::members::find_contact,],
        )
        .attach(db::Conn::fairing())
}
