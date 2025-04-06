pub static SELECT_USER_PASSWORD_HASH: &str = r#"
    SELECT password_hash FROM division_online.i_uses
        WHERE username = ?
            ALLOW FILTERING;
"#;

pub static SELECT_USER_USERNAME: &str = r#"
    SELECT username FROM division_online.i_uses
        WHERE username = ?;
"#;



pub static INSERT_SERVER: &str = r#"
    INSERT INTO division_online.o_servers(sid, desc, name, owner) 
        VALUES(?,?,?,?);
"#;


pub static INSERT_NEW_USER: &str = r#"
    INSERT INTO division_online.i_uses (username, password_hash)
        VALUES (?,?);
"#;

pub static SELECT_USER_SESSIONS: &str = r#"
    SELECT session FROM division_online.i_session_users
        WHERE username = ?
            ALLOW FILTERING;
"#;

pub static SELECT_SESSION: &str = r#"
    SELECT name, session, owner, config FROM division_online.i_sessions 
        WHERE session = ?
            ALLOW FILTERING;
"#;

