use crate::db;
use crate::api::structures;
use crate::security;


#[actix_web::post("/api/fetch_session_data")]
pub async fn fetch_session_data(
    session: actix_web::web::Data<security::structures::ScyllaSession>,
    form: actix_web::web::Json<structures::UserHolder>
) -> impl actix_web::Responder {

  
    let scylla_session = session.lock.lock().unwrap();
    match db::sessions::fetch_user_session_data(&scylla_session, form.username.clone()).await {
        Some(v) => {
            actix_web::HttpResponse::Ok().json(
                &structures::SessionData {
                    sessions: v
                }
            )
        },
        _ => {
            actix_web::HttpResponse::Ok().json(
                &structures::SessionData {
                    sessions: Vec::new()
                }
            ) 
        }
    }   
}

#[actix_web::post("/api/fetch_session_message")]
pub async fn fetch_session_messages(
    session: actix_web::web::Data<security::structures::ScyllaSession>,
    form: actix_web::web::Json<structures::SessionHolder>
) -> impl actix_web::Responder {

  
    let scylla_session = session.lock.lock().unwrap();
    match db::messages::fetch_session_messages(&scylla_session, form.session.clone()).await {
        Some(v) => {
            actix_web::HttpResponse::Ok().json(
                &structures::SessionMessages {
                    messages: v
                }
            )
        },
        _ => {
            actix_web::HttpResponse::Ok().json(
                &structures::SessionMessages {
                    messages: Vec::new()
                }
            ) 
        }
    }   
}

#[actix_web::post("/api/send_session_message")]
pub async fn send_session_message(
    session: actix_web::web::Data<security::structures::ScyllaSession>,
    form: actix_web::web::Json<structures::MessageSessionHolder>
) -> impl actix_web::Responder {

  
    let scylla_session = session.lock.lock().unwrap();
    match db::messages::send_session_message(&scylla_session, form.session.clone(), form.content.clone(), form.username.clone()).await {
        Ok(_) => {
            actix_web::HttpResponse::Ok().json(
                &structures::TokenHolder {
                    token: "ok".to_string()
                }
            )
        },
        _ => {
            actix_web::HttpResponse::Ok().json(
                &structures::TokenHolder {
                    token: "nok".to_string()
                }
            ) 
        }
    }   
}

#[actix_web::post("/api/fetch_session_notes")]
pub async fn fetch_session_notes(
    session: actix_web::web::Data<security::structures::ScyllaSession>,
    form: actix_web::web::Json<structures::SessionHolder>
) -> impl actix_web::Responder {

  
    let scylla_session = session.lock.lock().unwrap();
    match db::sessions::fetch_session_notes(&scylla_session, form.session.clone()).await {
        Some(v) => {
            actix_web::HttpResponse::Ok().json(
                &structures::SessionNotes {
                    notes: v
                }
            )
        },
        _ => {
            actix_web::HttpResponse::Ok().json(
                &structures::SessionNotes {
                    notes: Vec::new()
                }
            ) 
        }
    }   
}

#[actix_web::post("/api/new_session_note")]
pub async fn new_session_note(
    session: actix_web::web::Data<security::structures::ScyllaSession>,
    form: actix_web::web::Json<structures::NoteSessionHolder>
) -> impl actix_web::Responder {

  
    let scylla_session = session.lock.lock().unwrap();
    match db::sessions::new_session_note(&scylla_session,
        form.session.clone(),
        form.content.clone(),
        form.username.clone(),
        form.description.clone(),
        form.title.clone())
    .await {
        Ok(_) => {
            actix_web::HttpResponse::Ok().json(
                &structures::TokenHolder {
                    token: "ok".to_string()
                }
            )
        },
        _ => {
            actix_web::HttpResponse::Ok().json(
                &structures::TokenHolder {
                    token: "nok".to_string()
                }
            ) 
        }
    }   
}

