use db_core::DatabaseConfig;
use serde::Deserialize;
use toolcraft_config::load_settings;
use toolcraft_jwt::{VerifyJwt, VerifyJwtCfg};
use toolcraft_request::{HeaderMap, Request};

use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpCfg,
    pub jwt_verify: JwtVerifyRemoteCfg,
    pub db: Vec<DatabaseConfig>,
}

#[derive(Debug, Deserialize)]
pub struct HttpCfg {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct JwtVerifyRemoteCfg {
    pub url: String,
    pub header: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
struct JwtVerifyConfigResponse {
    code: i32,
    data: JwtVerifyConfigData,
    message: String,
}

#[derive(Debug, Deserialize)]
struct JwtVerifyConfigData {
    public_key_pem: String,
    issuer: String,
    audience: String,
}

impl Settings {
    pub fn load(config_path: &str) -> Result<Self> {
        let r = load_settings(config_path)?;
        Ok(r)
    }
}

impl JwtVerifyRemoteCfg {
    pub async fn fetch_verify_jwt(&self) -> Result<VerifyJwt> {
        let client = Request::new()
            .map_err(|e| crate::error::Error::Custom(format!("request client init failed: {e}")))?;

        let mut headers = HeaderMap::new();
        headers
            .insert(self.header.clone(), self.token.clone())
            .map_err(|e| crate::error::Error::Custom(format!("invalid jwt verify header: {e}")))?;

        let response = client
            .get(&self.url, None, Some(headers))
            .await
            .map_err(|e| {
                crate::error::Error::Custom(format!("fetch jwt verify config failed: {e}"))
            })?;

        let status = response.status();
        let payload: JwtVerifyConfigResponse = response.json().await.map_err(|e| {
            crate::error::Error::Custom(format!("decode jwt verify config failed: {e}"))
        })?;

        if !status.is_success() {
            return Err(crate::error::Error::Custom(format!(
                "fetch jwt verify config failed: status={}, message={}",
                status, payload.message
            )));
        }

        if payload.code != 0 {
            return Err(crate::error::Error::Custom(format!(
                "fetch jwt verify config failed: code={}, message={}",
                payload.code, payload.message
            )));
        }

        VerifyJwt::new(VerifyJwtCfg {
            public_key_pem: payload.data.public_key_pem,
            issuer: payload.data.issuer,
            audience: payload.data.audience,
        })
        .map_err(Into::into)
    }
}
