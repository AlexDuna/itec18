pub static SELECT_USER_PASSWORD_HASH: &str = r#"
    SELECT password_hash FROM division_online.i_users
        WHERE username = ?
            ALLOW FILTERING;
"#;

pub static SELECT_USER_SESSIONS: &str = r#"
    SELECT session FROM division_online.i_session_users
        WHERE username = ?
            ALLOW FILTERING;
"#;


