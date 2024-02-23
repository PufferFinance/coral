use axum::http::StatusCode;
use std::collections::HashMap;

use crate::{
    add_0x_prefix,
    services::beacon::types::{StateId, ValidatorId},
};

use super::{
    client::BeaconClientTrait,
    types::{ValidatorData, ValidatorListResponse},
};
use crate::{
    error::{AppServerResult, ServerErrorResponse},
    utils::encoding::parse_json_response,
};

pub async fn fetch_validators_by_index(
    beacon_url: &str,
    state_id: StateId,
    validators: &[u64],
) -> AppServerResult<ValidatorListResponse> {
    let validators_query: String = validators
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let api_url =
        format!("{beacon_url}/eth/v1/beacon/states/{state_id}/validators?id={validators_query}");
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
        tracing::error!("Failed to fetch validators");
        let status_code = StatusCode::from_u16(status_code.as_u16()).unwrap();
        let err = ServerErrorResponse::new(status_code, 1000, &format!("{}", status_code));
        return Err(err);
    }

    let resp_json = parse_json_response(resp).await?;
    Ok(resp_json)
}

pub async fn fetch_validators_by_pubkey(
    beacon_url: &str,
    state_id: StateId,
    validators: &[String],
) -> AppServerResult<ValidatorListResponse> {
    let validators_query: String = validators.to_vec().join(",");

    let api_url =
        format!("{beacon_url}/eth/v1/beacon/states/{state_id}/validators?id={validators_query}");
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
        tracing::error!("Failed to fetch validators");
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

pub async fn fetch_validators_fallback(
    beacon_client: &impl BeaconClientTrait,
    validator_pubkeys: &[String],
) -> HashMap<u64, ValidatorData> {
    let validator_pubkeys: Vec<String> =
        validator_pubkeys.iter().map(|k| add_0x_prefix(k)).collect();

    let validators_by_index: HashMap<u64, ValidatorData> = match beacon_client
        .fetch_validators_by_pubkey(StateId::Finalized, validator_pubkeys.as_slice())
        .await
    {
        Ok(validators) => validators.data.into_iter().map(|v| (v.index, v)).collect(),
        Err(_err) => {
            tracing::debug!("Failed to fetch validators in bulk. Fetching individually instead");

            let mut validators_by_index: HashMap<u64, ValidatorData> = HashMap::new();
            for pubkey in validator_pubkeys {
                let validator_resp = beacon_client
                    .fetch_validator(StateId::Finalized, ValidatorId::Pubkey(pubkey.clone()))
                    .await;
                if let Ok(v) = validator_resp {
                    validators_by_index.insert(v.data.index, v.data);
                }
            }
            validators_by_index
        }
    };
    validators_by_index
}
