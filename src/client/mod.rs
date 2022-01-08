mod authenticator;
mod endpoint;
pub mod types_bootstrap;
pub mod types_image;
pub mod types_update_camera;

use crate::client::authenticator::Authenticator;
use crate::client::endpoint::Endpoint;
use crate::client::types_bootstrap::{BootstrapResponse, Camera};
use crate::client::types_image::ImageResponse;
use crate::client::types_update_camera::UpdateCameraRequest;
use crate::config::Unifi;
use actix_web::http::HeaderValue;
use anyhow::{ensure, Result};
use chrono::Duration;
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::StatusCode;

#[derive(Debug, Clone)]
pub struct ApiClient {
    authenticator: Authenticator,
    endpoint: Endpoint,
}

impl ApiClient {
    pub fn init(cfg: &Unifi) -> Result<ApiClient> {
        Ok(ApiClient {
            authenticator: Authenticator::init(cfg)?,
            endpoint: Endpoint::new(cfg.ip.to_string()),
        })
    }

    fn bootstrap(&self) -> Result<BootstrapResponse> {
        let client = self
            .http_client()?
            .get(self.endpoint.get("/proxy/protect/api/bootstrap"));
        let res = self.send_with_session(client)?;
        let bootstrap: BootstrapResponse = res.json()?;
        Ok(bootstrap)
    }

    pub fn get_doorbells(&self) -> Result<Vec<Camera>> {
        Ok(self
            .bootstrap()?
            .cameras
            .into_iter()
            .filter(|camera| camera.is_doorbell())
            .collect())
    }

    pub fn get_snapshot(&self, doorbell_id: impl Into<String>) -> Result<ImageResponse> {
        let client = self.http_client()?.get(self.endpoint.get(format!(
            "/proxy/protect/api/cameras/{}/snapshot",
            doorbell_id.into()
        )));
        let res = self.send_with_session(client)?;

        let content_type = res
            .headers()
            .get("content-type")
            .unwrap_or(&HeaderValue::from_static("image/jpeg"))
            .to_str()
            .unwrap()
            .to_string();
        Ok(ImageResponse {
            content: res.bytes()?,
            content_type,
        })
    }

    pub fn set_message(
        &self,
        doorbell_id: impl Into<String>,
        message: impl Into<String>,
        duration: Duration,
    ) -> Result<()> {
        let client = self
            .http_client()?
            .patch(
                self.endpoint
                    .get(format!("/proxy/protect/api/cameras/{}", doorbell_id.into())),
            )
            .json(&UpdateCameraRequest::update_message(message, duration));
        let res = self.send_with_session(client)?;
        ensure!(
            res.status() == StatusCode::OK,
            "failed to update camera. status: {} err: {}",
            res.status(),
            res.text()?
        );
        Ok(())
    }

    fn http_client(&self) -> reqwest::Result<Client> {
        reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
    }

    fn send_with_session(&self, req_builder: RequestBuilder) -> Result<Response> {
        let (cookie, csrf_token) = self.authenticator.get_cookie_with_csrf_token()?;
        Ok(req_builder
            .header("cookie", cookie)
            .header("x-csrf-token", csrf_token)
            .send()?)
    }
}
