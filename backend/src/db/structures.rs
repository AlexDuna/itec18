#[derive(scylla::SerializeRow)]
pub struct UserUsername {
    pub username: Option<String>
}

#[derive(Debug, scylla::SerializeValue, scylla::DeserializeValue)]
pub struct User {
    pub username: Option<String>,
    pub password_hash: Option<String>,
}

impl User {
    
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            username: Some(username),
            password_hash: Some(password_hash),
        }
    }

}

#[derive(scylla::SerializeRow)]
pub struct SessionHolder {
    pub session: Option<String>
}

#[derive(scylla::SerializeRow, Clone,serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub session: Option<String>,
    pub config: Option<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub tag: Option<String>
}

#[derive(scylla::SerializeRow, Clone,serde::Serialize, serde::Deserialize)]
pub struct MessageHolder {
    pub content: Option<String>,
    pub username: Option<String>
}
