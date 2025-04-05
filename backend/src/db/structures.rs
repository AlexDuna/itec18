#[derive(scylla::SerializeRow)]
pub struct UserUsername {
    pub username: Option<String>
}
