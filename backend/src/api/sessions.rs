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
