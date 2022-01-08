use crate::client::ApiClient;
use crate::config::Configuration;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn service() -> Scope {
    web::scope("/api")
        .app_data(web::JsonConfig::default())
        .service(get_snapshot)
        .service(get_message_templates)
        .service(set_message)
}

pub fn wellknown_service() -> Scope {
    web::scope("/.well-known").service(wellknown_configuration)
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
}

fn internal_server_error_response() -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse {
        message: "internal server error occurred.".to_string(),
    })
}

#[derive(Serialize)]
struct WellknownConfigurationResponse {
    api_endpoint: String,
}

#[get("/configuration")]
async fn wellknown_configuration(cfg: web::Data<Configuration>) -> HttpResponse {
    HttpResponse::Ok().json(WellknownConfigurationResponse {
        api_endpoint: format!("http://localhost:{}/api", cfg.server.port),
    })
}

#[get("/snapshot/{doorbell_id}")]
async fn get_snapshot(
    client: web::Data<Arc<ApiClient>>,
    web::Path(doorbell_id): web::Path<String>,
) -> impl Responder {
    match client.get_snapshot(doorbell_id) {
        Ok(image) => HttpResponse::Ok()
            .content_type(image.content_type)
            .body(image.content.to_vec()),
        Err(err) => {
            error!("{}", err);
            internal_server_error_response()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SetMessageRequest {
    doorbell_id: String,
    message: String,
    duration_sec: i64,
}

#[post("/message/set")]
async fn set_message(
    client: web::Data<Arc<ApiClient>>,
    req: web::Json<SetMessageRequest>,
) -> impl Responder {
    match client.set_message(
        req.doorbell_id.as_str(),
        req.message.as_str(),
        Duration::seconds(req.duration_sec),
    ) {
        Ok(..) => HttpResponse::Created().finish(),
        Err(err) => {
            error!("{}", err);
            internal_server_error_response()
        }
    }
}

#[derive(Serialize)]
struct TemplateListResponse {
    templates: Vec<String>,
}

#[get("/message/templates")]
async fn get_message_templates(cfg: web::Data<Configuration>) -> impl Responder {
    let resp = TemplateListResponse {
        templates: cfg.message.templates.clone(),
    };
    HttpResponse::Ok().json(resp)
}
