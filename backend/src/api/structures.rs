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
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SessionData {
    pub sessions: Vec<db::structures::Session>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SessionMessages {
    pub messages: Vec<db::structures::MessageHolder>
}

#[derive(serde::Deserialize)]
pub struct SessionHolder {
    pub session: String
}

#[derive(serde::Deserialize)]
pub struct MessageSessionHolder {
    pub session: String,
    pub username: String,
    pub content: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SessionNotes {
    pub notes: Vec<db::structures::SessionNote>
}
#[derive(serde::Deserialize)]
pub struct NoteSessionHolder {
    pub session: String, 
    pub content: String,
    pub username: String, 
    pub description: String,
    pub title: String
}
