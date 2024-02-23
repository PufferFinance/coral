use axum::http::StatusCode;

use crate::services::beacon::types::StateId;
use crate::{
    error::{AppServerResult, ServerErrorResponse},
    utils::encoding::parse_json_response,
};

use super::types::{ValidatorId, ValidatorResponse};

pub async fn fetch_validator_by_index(
    beacon_url: &str,
    state_id: StateId,
    validator_index: u64,
) -> AppServerResult<ValidatorResponse> {
    let api_url =
        format!("{beacon_url}/eth/v1/beacon/states/{state_id}/validators/{validator_index}");
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
        tracing::error!("Failed to fetch validator");
        let status_code = StatusCode::from_u16(status_code.as_u16()).unwrap();
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }

    let resp_json = parse_json_response(resp).await?;
    Ok(resp_json)
}

pub async fn fetch_validator(
    beacon_url: &str,
    state_id: StateId,
    validator: ValidatorId,
) -> AppServerResult<ValidatorResponse> {
    let api_url = format!("{beacon_url}/eth/v1/beacon/states/{state_id}/validators/{validator}");
    tracing::debug!("Beacon: {api_url}");
    let resp = reqwest::Client::new()
        .request(reqwest::Method::GET, api_url)
        .send()
        .await
        .map_err(|err| {
            let error_msg = "Failed to send request";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;

    let status_code = resp.status();
    if !status_code.is_success() {
        tracing::error!("Failed to fetch validator {validator}");
        let body = resp.text().await.map_err(|err| {
            let error_msg = "Failed to parse body";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;
        eprintln!("{body}");
        let status_code = StatusCode::from_u16(status_code.as_u16()).unwrap();
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }

    let resp_json = parse_json_response(resp).await?;
    Ok(resp_json)
}
