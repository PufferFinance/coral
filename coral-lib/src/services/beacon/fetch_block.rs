use axum::http::StatusCode;

use crate::services::beacon::types::{BlockResponse, BlockRootResponse};
use crate::{
    error::{AppServerResult, ServerErrorResponse},
    utils::encoding::parse_json_response,
};

use super::types::BlockId;

pub async fn fetch_block(beacon_url: &str, block_id: BlockId) -> AppServerResult<BlockResponse> {
    let api_url = format!("{beacon_url}/eth/v2/beacon/blocks/{block_id}");
    tracing::debug!("Beacon: {api_url}");
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
        let body = resp.text().await.map_err(|err| {
            let error_msg = "Failed to parse body";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;
        eprintln!("{body}");
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }
    let resp_json = parse_json_response(resp).await?;
    Ok(resp_json)
}

pub async fn fetch_block_root(beacon_url: &str, block_id: BlockId) -> AppServerResult<String> {
    let api_url = format!("{beacon_url}/eth/v1/beacon/blocks/{block_id}/root");
    tracing::debug!("Beacon: {api_url}");
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
        let body = resp.text().await.map_err(|err| {
            let error_msg = "Failed to parse body";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;
        eprintln!("{body}");
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }
    let resp_json: BlockRootResponse = parse_json_response(resp).await?;
    Ok(resp_json.data.root)
}
