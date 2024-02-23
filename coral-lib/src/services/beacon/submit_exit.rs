use axum::http::StatusCode;
use serde::Serialize;

use crate::error::{AppServerResult, ServerErrorResponse};

#[derive(Clone, Debug, Serialize)]
pub struct VoluntaryExit {
    pub epoch: String,
    pub validator_index: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: String,
}

pub async fn submit_exit(
    beacon_url: &str,
    validator_index: u64,
    epoch: u64,
    signature: &str,
) -> AppServerResult<()> {
    let req_body = SignedVoluntaryExit {
        message: VoluntaryExit {
            epoch: epoch.to_string(),
            validator_index: validator_index.to_string(),
        },
        signature: signature.to_string(),
    };
    let req_body_str = serde_json::to_string(&req_body).unwrap();

    eprintln!("Req Body: {}", req_body_str);

    let api_url = format!("{beacon_url}/eth/v1/beacon/pool/voluntary_exits");
    let resp = reqwest::Client::new()
        .request(reqwest::Method::POST, api_url)
        .body(req_body_str)
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Failed to send request");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to send request",
            )
        })?;

    let status_code = resp.status();
    if !status_code.is_success() {
        let body = resp.text().await;
        eprintln!("Body: {:?}", body);
        let status_code = StatusCode::from_u16(status_code.as_u16()).unwrap();
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }
    Ok(())
}
