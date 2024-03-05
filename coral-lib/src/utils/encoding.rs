use axum::http::StatusCode;
use base64::Engine;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::error::{AppServerResult, ServerErrorCode, ServerErrorResponse};

pub fn base64_encode_to_bytes(s: &[u8]) -> AppServerResult<String> {
    let res = base64::engine::general_purpose::STANDARD.encode(s);
    Ok(res)
}

pub fn base64_decode_to_bytes(s: &str) -> AppServerResult<Vec<u8>> {
    let res = base64::engine::general_purpose::STANDARD
        .decode(s.as_bytes())
        .map_err(|err| {
            let error_msg = "Failed to base64 decode";
            tracing::error!("{error_msg}: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::ParseError,
                err.to_string(),
            )
        })?;
    Ok(res)
}

pub fn base64_decode_to_string(s: &str) -> AppServerResult<String> {
    let vec_bytes = base64_decode_to_bytes(s)?;
    let new_string = String::from_utf8(vec_bytes).map_err(|err| {
        let error_msg = "Failed to decode to utf8";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            err.to_string(),
        )
    })?;

    Ok(new_string)
}

pub async fn parse_json_response<T: DeserializeOwned>(resp: Response) -> AppServerResult<T> {
    let resp_json: T = resp.json().await.map_err(|err| {
        let error_msg = "Failed to parse JSON body";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            err.to_string(),
        )
    })?;
    Ok(resp_json)
}
