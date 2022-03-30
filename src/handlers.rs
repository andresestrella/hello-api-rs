use actix_web::{web, Responder, HttpResponse};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from API")
}

pub async fn hello_name(path: web::Path<(String,String)>) -> impl Responder {
    let name = path.into_inner();
    let s = format!("Hello {} from API", name.0);
    HttpResponse::Ok().json(s)
}

pub async fn hello_post_name(req_body: String) -> impl Responder {
    let s = format!("Hello {} from API", req_body);
    HttpResponse::Ok().json(s)
}