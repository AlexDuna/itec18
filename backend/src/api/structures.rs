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
