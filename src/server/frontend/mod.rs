use actix_web::body::Body;
use actix_web::{get, HttpRequest, HttpResponse};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "src/server/frontend/assets"]
struct Assets;

#[get("/{filename:.*}")]
pub async fn spa(req: HttpRequest) -> HttpResponse {
    let req_path = req.path().strip_prefix("/").unwrap_or("index.html");
    match get_file_response(req_path) {
        Some(response) => response,
        None => {
            get_file_response("index.html").unwrap_or(HttpResponse::NotFound().body("not found"))
        }
    }
}

fn get_file_response(path: &str) -> Option<HttpResponse> {
    match Assets::get(path) {
        Some(content) => {
            let body: Body = match content.data {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            Some(
                HttpResponse::Ok()
                    .content_type(from_path(path).first_or_octet_stream().as_ref())
                    .body(body),
            )
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, App, Error};

    #[actix_rt::test]
    async fn un_exits_path() -> Result<(), Error> {
        let app = App::new().service(spa);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
        Ok(())
    }

    #[actix_rt::test]
    async fn exists_path() -> Result<(), Error> {
        let app = App::new().service(spa);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/index.html").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
        Ok(())
    }
}
