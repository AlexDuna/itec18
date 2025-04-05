use crate::db::structures;
use crate::db::statics;

pub async fn fetch_user_sessions(
    session: &scylla::client::session::Session,
    username: String
) -> Option<Vec<structures::SessionHolder>> {
    let query_rows = session
        .query_unpaged(statics::SELECT_USER_SESSIONS, ((username),))
        .await.ok()?
        .into_rows_result().ok()?;
    let mut sessions = Vec::<structures::SessionHolder>::new();
    for row in query_rows.rows::<(Option<&str>,)>().ok()? {
        let (channel_name,): (Option<&str>,) = row.ok()?;
        match channel_name {
            Some(str) => {
                sessions.push(structures::SessionHolder{session: Some(str.to_string())});
            },
            None => {
                return None;
            }
        }
    }
    Some(sessions)
}


pub async fn fetch_session(
    session: &scylla::client::session::Session,
    sid: String
) -> Option<structures::Session> {
    let query_rows = session
        .query_unpaged(statics::SELECT_SESSION, ((sid),))
        .await.ok()?
        .into_rows_result().ok()?;
    for row in query_rows.rows::<(Option<&str>,Option<&str>,Option<&str>,Option<&str>)>().ok()?{
        match row.ok()? {
            ( Some(n),  Some(s), Some(o), Some(c) ) => {
                return Some(
                    structures::Session {
                        session: Some(s.to_string()),
                        config: Some(c.to_string()),
                        name: Some(n.to_string()),
                        owner: Some(o.to_string()),
                        tag: Some("".to_string())
                    }
                );
            },
            _ => {println!("?");return None;}
        };
    }
    None
}

pub async fn fetch_user_session_data(
    session: &scylla::client::session::Session,
    username: String
) -> Option<Vec<structures::Session>> {
    
    let mut retvec = Vec::new();
    match fetch_user_sessions(&session, username.clone()).await{
        Some(v) => {
            for session_id in v.iter() {
                match fetch_session(&session, session_id.session.clone()?).await {
                    Some(user_session) => {
                        retvec.push(user_session.clone());
                    },
                    _ => {}
                }
            }
        },
        _ => {
            println!("err");
            return None;
        }
    }
    Some(retvec)
}

