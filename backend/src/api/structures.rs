#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenHolder {
    pub token: String
}


#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String 
}

#[derive(serde::Deserialize)]
pub struct UserHolder {
    pub username: String
}

use crate::db;
#[derive(serde::Deserialize)]
pub struct SessionData {
    pub sessions: Vec<db::structures::Session>
}
