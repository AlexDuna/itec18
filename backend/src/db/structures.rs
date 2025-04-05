#[derive(scylla::SerializeRow)]
pub struct UserUsername {
    pub username: Option<String>
}

#[derive(Debug, scylla::SerializeValue, scylla::DeserializeValue)]
pub struct User {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

impl User {
    
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            username: Some(username),
            email: Some(email),
            password_hash: Some(password_hash),
        }
    }

}
