use crate::client::endpoint::Endpoint;
use crate::config::Unifi;
use anyhow::{bail, ensure, Result};
use chrono::{DateTime, Duration, TimeZone, Utc};
use jsonwebtoken::dangerous_insecure_decode;
use reqwest::cookie::Cookie;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
struct Session {
    key: String,
    token: String,
    claim: Claim,
}

impl Session {
    fn from_cookie(token: &Cookie) -> Result<Session> {
        Ok(Session {
            key: token.name().to_string(),
            token: token.value().to_string(),
            claim: Claim::from_token(token.value())?,
        })
    }

    fn cookie_string(&self) -> String {
        format!("{}={}", self.key, self.token)
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct Claim {
    csrf_token: String,
    // user_id: String,
    // remember_me: bool,
    // iat: i64,
    exp: i64,
}

impl Claim {
    const BUFFER_DURATION_SECONDS: i64 = 60;

    fn from_token(token: &str) -> Result<Claim> {
        Ok(dangerous_insecure_decode::<Claim>(token)?.claims)
    }

    fn expired_at(&self) -> DateTime<Utc> {
        Utc.timestamp(self.exp, 0)
    }

    fn will_soon_expire(&self) -> bool {
        let limit = self.expired_at() + Duration::seconds(Claim::BUFFER_DURATION_SECONDS);
        limit.timestamp() <= Utc::now().timestamp()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest {
    username: String,
    password: String,
    remember_me: bool,
}

#[derive(Debug, Clone)]
pub struct Authenticator {
    config: Unifi,
    endpoint: Endpoint,
    session: Arc<Mutex<Session>>,
    access_count: Arc<Mutex<u64>>,
}

impl Authenticator {
    pub(crate) fn init(cfg: &Unifi) -> Result<Authenticator> {
        let auth = Authenticator {
            config: cfg.clone(),
            endpoint: Endpoint::new(cfg.ip.to_string()),
            session: Arc::new(Mutex::new(Session::default())),
            access_count: Arc::new(Mutex::new(0)),
        };
        auth.authenticate()?;
        Ok(auth)
    }

    fn authenticate(&self) -> Result<()> {
        info!("authenticating...");
        let res = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .post(self.endpoint.get("/api/auth/login"))
            .json(&LoginRequest {
                username: self.config.username.clone(),
                password: self.config.password.clone(),
                remember_me: true,
            })
            .send()?;
        ensure!(
            res.status() == StatusCode::OK,
            "failed to authenticate. status: {} err: {}",
            res.status(),
            res.text()?
        );

        let token = res.cookies().find(|cookie| cookie.name() == "TOKEN");
        match token {
            Some(token) => {
                let mut cookie = self.session.lock().unwrap();
                *cookie = Session::from_cookie(&token)?;
                debug!("session will expire at: {:#?}", cookie.claim.expired_at());
                Ok(())
            }
            None => bail!("not found TOKEN at response cookie"),
        }
    }

    pub fn get_cookie_with_csrf_token(&self) -> Result<(String, String)> {
        let mut count = self.access_count.lock().unwrap();
        debug!("count: {}", *count);
        *count += 1;
        let mut session = self.session.lock().unwrap();
        if session.claim.will_soon_expire() {
            drop(session);
            self.authenticate()?;
            session = self.session.lock().unwrap();
        }
        Ok((
            session.cookie_string(),
            session.claim.csrf_token.to_string(),
        ))
    }
}
