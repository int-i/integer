#[derive(Queryable)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub note: Option<String>,
}
