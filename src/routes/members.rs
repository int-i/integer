use crate::db::Conn;
use crate::models::Member;
use crate::schema::members::dsl::*;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/<member_id>")]
pub async fn find(conn: Conn, member_id: i32) -> Option<Json<Member>> {
    conn.run(move |c| members.filter(id.eq(&member_id)).get_result(c))
        .await
        .map(Json)
        .ok()
}
