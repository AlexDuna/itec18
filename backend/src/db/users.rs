use crate::db::structures;
use crate::db::statics;

pub async fn get_user_password_hash(
    session: &scylla::client::session::Session,
    user: structures::UserUsername
) -> Option<String> {
    let query_rows = session
        .query_unpaged(statics::SELECT_USER_PASSWORD_HASH, ((user.username),))
        .await.ok()?
        .into_rows_result().ok()?;
    for row in query_rows.rows::<(Option<&str>,)>().ok()?{
        let (password_hash_str,): (Option<&str>,) = row.ok()?;
        match password_hash_str {
            Some(str) => {return Some(str.to_string());},
            _ => {println!("?");return None;}
        };
    }
    None
}
