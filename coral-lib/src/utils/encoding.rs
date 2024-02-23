use axum::http::StatusCode;
use base64::Engine;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::error::{AppServerResult, ServerErrorResponse};

pub fn base64_encode_to_bytes(s: &[u8]) -> AppServerResult<String> {
    let res = base64::engine::general_purpose::STANDARD.encode(s);
    Ok(res)
}

pub fn base64_decode_to_bytes(s: &str) -> AppServerResult<Vec<u8>> {
    let res = base64::engine::general_purpose::STANDARD
        .decode(s.as_bytes())
        .map_err(|err| {
            tracing::error!("Failed to base64 decode");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to decode base64 decode",
            )
        })?;
    Ok(res)
}

pub fn base64_decode_to_string(s: &str) -> AppServerResult<String> {
    let vec_bytes = base64_decode_to_bytes(s)?;
    let new_string = String::from_utf8(vec_bytes).map_err(|err| {
        tracing::error!("Failed to decode to utf8");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to decode decode to utf8",
        )
    })?;

    Ok(new_string)
}

pub async fn parse_json_response<T: DeserializeOwned>(resp: Response) -> AppServerResult<T> {
    if !resp.status().is_success() {
        let body = resp.text().await.map_err(|err| {
            tracing::error!("Failed to parse body");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to parse body",
            )
        })?;
        tracing::error!("Body: {}", body);
        let err = ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, &body);
        return Err(err);
    }

    let resp_json: T = resp.json().await.map_err(|err| {
        tracing::error!("Failed to parse JSON");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to parse JSON",
        )
    })?;
    Ok(resp_json)
}
