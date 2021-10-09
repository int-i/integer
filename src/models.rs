use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberPublic {
    pub id: i32,
    pub name: String,
    pub admission_year: i32,
}

impl MemberPublic {
    pub fn new(member: Member) -> Self {
        Self {
            id: member.id,
            name: member.name,
            admission_year: member.id / 10000 % 100,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberContact {
    pub id: i32,
    pub name: String,
    pub admission_year: i32,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl MemberContact {
    pub fn new(member: Member) -> Self {
        Self {
            id: member.id,
            name: member.name,
            admission_year: member.id / 10000 % 100,
            phone: member.phone,
            email: member.email,
        }
    }
}
