use serde::Deserialize;
use toolcraft_jwt::{VerifyJwt, VerifyJwtCfg};
use crate::{
    error::{Error, Result},
    settings::JwtVerifyRemoteCfg,
    statics::request_client::get_request_client,
};

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

pub async fn fetch_verify_jwt(cfg: &JwtVerifyRemoteCfg) -> Result<VerifyJwt> {
    let response = get_request_client()
        .get(&cfg.url, None, None)
        .await
        .map_err(|e| Error::Custom(format!("fetch jwt verify config failed: {e}")))?;

    let status = response.status();
    let payload: JwtVerifyConfigResponse = response
        .json()
        .await
        .map_err(|e| Error::Custom(format!("decode jwt verify config failed: {e}")))?;

    if !status.is_success() {
        return Err(Error::Custom(format!(
            "fetch jwt verify config failed: status={}, message={}",
            status, payload.message
        )));
    }

    if payload.code != 0 {
        return Err(Error::Custom(format!(
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
