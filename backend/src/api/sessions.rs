use crate::db;
use crate::api::structures;
use crate::security;


#[actix_web::post("/api/try_login")]
pub async fn try_login(
    session: actix_web::web::Data<security::structures::ScyllaSession>,
    form: actix_web::web::Json<structures::LoginUser>
) -> impl actix_web::Responder {

  
    let username = db::structures::UserUsername {
        username: Some(form.username.clone())
    };
    let scylla_session = session.lock.lock().unwrap();
    match db::sessions::fetch_user_session_data(&scylla_session, form.username.clone).await {
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
