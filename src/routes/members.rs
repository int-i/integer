use crate::db::Conn;
use crate::models::*;
use crate::schema::members::dsl::*;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/<member_id>")]
pub async fn find(conn: Conn, member_id: i32) -> Option<Json<MemberPublic>> {
    conn.run(move |c| members.filter(id.eq(&member_id)).get_result::<Member>(c))
        .await
        .map(MemberPublic::new)
        .map(Json)
        .ok()
}

#[get("/<member_id>/contacts")]
pub async fn find_contact(conn: Conn, member_id: i32) -> Option<Json<MemberContact>> {
    conn.run(move |c| members.filter(id.eq(&member_id)).get_result::<Member>(c))
        .await
        .map(MemberContact::new)
        .map(Json)
        .ok()
}
