use actix_web::HttpResponse;
use log;
#[path="./db.rs"] mod db;

pub async fn add_kernel() -> HttpResponse {
    HttpResponse::Ok().body("Add kernel")
}
pub async fn list_kernel() -> HttpResponse {
    HttpResponse::Ok().body("List kernel")
}
pub async fn update_kernel() -> HttpResponse {
    HttpResponse::Ok().body("Update kernel")
}
pub async fn delete_kernel() -> HttpResponse {
    HttpResponse::Ok().body("Delete kernel")
}

pub async fn add_request() -> HttpResponse {
    HttpResponse::Ok().body("Add request")
}
pub async fn list_request() -> HttpResponse {
    HttpResponse::Ok().body("List request")
}
pub async fn update_request() -> HttpResponse {
    HttpResponse::Ok().body("updte request")
}
pub async fn delete_request() -> HttpResponse {
    HttpResponse::Ok().body("Delete request")
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login")
}