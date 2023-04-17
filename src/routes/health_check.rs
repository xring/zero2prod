use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    tracing::info!("Health check is ok!");
    HttpResponse::Ok().finish()
}
