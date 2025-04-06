use crate::db::structures;
use crate::db::statics;
use crate::security;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn fetch_session_messages(
    session: &scylla::client::session::Session,
    ses: String
) -> Option<Vec<structures::MessageHolder>> {
    let query_rows = session
        .query_unpaged(statics::SELECT_SESSION_MESSAGES, ((ses),))
        .await.ok()?
        .into_rows_result().ok()?;
    let mut messages = Vec::<structures::MessageHolder>::new();
    for row in query_rows.rows::<(Option<&str>,Option<&str>)>().ok()? {
        match row.ok()? {
            (Some(c), Some(u)) => {
                messages.push(structures::MessageHolder{
                    content: Some(c.to_string()),
                    username: Some(u.to_string())
                });
            },
            _ => {
                return None;
            }
        }
    }
    Some(messages)
}

pub async fn send_session_message(
    session: &scylla::client::session::Session,
    ses: String,
    username: String,
    content: String
) -> Result<()> {
    return session
        .query_unpaged(statics::INSERT_MESSAGE, (security::token(), ses.clone(), content.clone(), username.clone()))
        .await
        .map(|_|())
        .map_err(From::from);
}
