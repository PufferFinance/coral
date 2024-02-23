use axum::http::StatusCode;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    utils::encoding::parse_json_response,
};

use super::types::AttestationRewardsResponse;

pub async fn get_attestation_rewards(
    beacon_url: &str,
    epoch: u64,
    validators: &[u64],
) -> AppServerResult<AttestationRewardsResponse> {
    let validators_json_body: String = serde_json::to_string(
        &validators
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    )
    .unwrap();

    let api_url = format!("{beacon_url}/eth/v1/beacon/rewards/attestations/{epoch}");
    let resp = reqwest::Client::new()
        .request(reqwest::Method::POST, api_url)
        .body(validators_json_body)
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

    let resp_json: AttestationRewardsResponse = parse_json_response(resp).await?;
    Ok(resp_json)
}
