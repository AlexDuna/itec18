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
    match db::users::get_user_password_hash(&scylla_session, username).await {
        Some(password_hash) => {
            let user_password_hash = security::sha512(form.password.clone());
            if user_password_hash == password_hash {
                actix_web::HttpResponse::Ok().json(
                    &structures::TokenHolder {
                        token: "ok".to_string()
                    }

                )
            } else {
                println!("not matchy");
                actix_web::HttpResponse::Ok().json(
                    &structures::TokenHolder {
                        token: "".to_string()
                    }
                )
            }
        },
        None => {
            println!("no hash");
            actix_web::HttpResponse::Ok().json(
                &structures::TokenHolder {
                    token: "".to_string()
                }
            )
        }
    }   
}
