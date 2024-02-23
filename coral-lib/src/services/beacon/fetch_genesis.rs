use axum::http::StatusCode;

use serde::Deserialize;

use crate::services::beacon::types::BeaconGenesis;
use crate::{
    error::{AppServerResult, ServerErrorResponse},
    utils::encoding::parse_json_response,
};

#[derive(Clone, Debug, Deserialize)]
pub struct BeaconGenesisWrapper {
    pub data: BeaconGenesis,
}

pub async fn fetch_genesis(beacon_url: &str) -> AppServerResult<BeaconGenesis> {
    let api_url = format!("{beacon_url}/eth/v1/beacon/genesis");
    let resp = reqwest::Client::new()
        .request(reqwest::Method::GET, api_url)
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
        let status_code = StatusCode::from_u16(status_code.as_u16()).unwrap();
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }
    let resp_json: BeaconGenesisWrapper = parse_json_response(resp).await?;
    Ok(resp_json.data)
}
