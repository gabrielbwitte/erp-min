use actix_web::{HttpResponse, Responder, web::Json};

use crate::dto::auth::LoginRequest;

pub async fn login(data: Json<LoginRequest>) -> impl Responder {
    let data = data.into_inner();
    HttpResponse::Ok().json(data)
}
