use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub note: Option<String>,
}
