use actix_web::{get, web, Error, HttpResponse};

#[get("/health")]
pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("OK!"))
}

pub fn init_health_routes(config: &mut web::ServiceConfig) {
    config.service(health);
}
