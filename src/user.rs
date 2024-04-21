use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub headline: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Role,
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "supervisor")]
    Supervisor,
    #[serde(rename = "member")]
    Member,
}

///
#[derive(Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "removed")]
    Removed,
}
