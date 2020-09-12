use actix_web::HttpResponse;

pub async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}
