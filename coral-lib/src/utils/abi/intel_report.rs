use ethers::abi;
use ethers::utils::hex::ToHex;

use axum::http::StatusCode;
use puffersecuresigner::io::remote_attestation::AttestationReport;

use crate::error::{AppServerResult, ServerErrorResponse};
use crate::utils;

pub fn deserialize_report(b: &[u8]) -> AppServerResult<AttestationReport> {
    let report: AttestationReport = serde_json::from_slice(b).map_err(|err| {
        tracing::error!("Failed to parse attestation report");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to parse attestation report",
        )
    })?;
    Ok(report)
}

pub fn to_json(report: &AttestationReport) -> AppServerResult<String> {
    let decoded_quote_body = utils::encoding::base64_decode_to_bytes(&report.isvEnclaveQuoteBody)?;
    let advisory_ids = serde_json::to_string(&report.advisoryIDs).map_err(|err| {
        tracing::error!("Failed to serialize advisoryIDs");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to serialize advisoryIDs",
        )
    })?;

    let json_data = serde_json::json!({
        "id": report.id,
        "timestamp": report.timestamp,
        "version": report.version,
        "epidPseudonym": report.epidPseudonym,
        "advisoryURL": report.advisoryURL,
        "advisoryIds": advisory_ids,
        "isvEnclaveQuoteStatus": report.isvEnclaveQuoteStatus,
        "isvEnclaveQuoteBody": decoded_quote_body.encode_hex::<String>(),
    });

    let json_data = serde_json::json!({
        "report": json_data
    });

    let json_string = serde_json::to_string(&json_data).map_err(|err| {
        tracing::error!("Failed to serialize json");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to serialize json",
        )
    })?;
    Ok(json_string)
}

pub fn to_calldata_with_decoded_quote_body(
    attestation_report: &AttestationReport,
) -> AppServerResult<abi::Bytes> {
    let decoded_quote_body =
        utils::encoding::base64_decode_to_bytes(&attestation_report.isvEnclaveQuoteBody)?;

    let advisory_ids = serde_json::to_string(&attestation_report.advisoryIDs).map_err(|err| {
        tracing::error!("Failed to serialize advisoryIDs");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to serialize advisoryIDs",
        )
    })?;

    let calldata = abi::encode(&[
        abi::Token::Bytes(attestation_report.id.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.timestamp.as_bytes().to_vec()),
        abi::Token::Bytes(
            format!("{}", attestation_report.version)
                .as_bytes()
                .to_vec(),
        ),
        abi::Token::Bytes(attestation_report.epidPseudonym.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.advisoryURL.as_bytes().to_vec()),
        abi::Token::Bytes(advisory_ids.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.isvEnclaveQuoteStatus.as_bytes().to_vec()),
        abi::Token::Bytes(decoded_quote_body.to_vec()),
    ]);
    Ok(calldata)
}

pub fn to_calldata(attestation_report: &AttestationReport) -> AppServerResult<abi::Bytes> {
    let advisory_ids = serde_json::to_string(&attestation_report.advisoryIDs).map_err(|err| {
        tracing::error!("Failed to serialize advisoryIDs");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to serialize advisoryIDs",
        )
    })?;

    let calldata = abi::encode(&[
        abi::Token::Bytes(attestation_report.id.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.timestamp.as_bytes().to_vec()),
        abi::Token::Bytes(
            format!("{}", attestation_report.version)
                .as_bytes()
                .to_vec(),
        ),
        abi::Token::Bytes(attestation_report.epidPseudonym.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.advisoryURL.as_bytes().to_vec()),
        abi::Token::Bytes(advisory_ids.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.isvEnclaveQuoteStatus.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.isvEnclaveQuoteBody.as_bytes().to_vec()),
    ]);
    Ok(calldata)
}

pub fn from_calldata(data: &[u8]) -> AppServerResult<AttestationReport> {
    let calldata_tokens = abi::decode(
        &[
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
        ],
        data,
    )
    .map_err(|err| {
        let error_msg = "Failed to parse report calldata";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;

    let report_data = match calldata_tokens.as_slice() {
        [abi::Token::Bytes(id), abi::Token::Bytes(timestamp), abi::Token::Bytes(version), abi::Token::Bytes(epid_pseudonym), abi::Token::Bytes(advisory_url), abi::Token::Bytes(advisory_ids), abi::Token::Bytes(isv_enclave_quote_status), abi::Token::Bytes(encoded_quote_body), ..] =>
        {
            let id = String::from_utf8(id.clone()).map_err(|err| {
                let error_msg = "Failed to decode report calldata fields";
                tracing::error!("{error_msg}");
                tracing::error!("{err}");
                ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
            })?;

            let timestamp = String::from_utf8(timestamp.clone()).map_err(|err| {
                let error_msg = "Failed to decode report calldata fields";
                tracing::error!("{error_msg}");
                tracing::error!("{err}");
                ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
            })?;

            let version: u32 = {
                let s = String::from_utf8(version.clone()).map_err(|err| {
                    let error_msg = "Failed to decode report calldata fields";
                    tracing::error!("{error_msg}");
                    tracing::error!("{err}");
                    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
                })?;
                s.parse::<u32>().map_err(|err| {
                    let error_msg = "Failed to decode report calldata fields";
                    tracing::error!("{error_msg}");
                    tracing::error!("{err}");
                    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
                })?
            };

            let epid_pseudonym = String::from_utf8(epid_pseudonym.clone()).map_err(|err| {
                let error_msg = "Failed to decode report calldata isv_enclave_quote_status";
                tracing::error!("{error_msg}");
                tracing::error!("{err}");
                ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
            })?;

            let advisory_url = String::from_utf8(advisory_url.clone()).map_err(|err| {
                let error_msg = "Failed to decode report calldata isv_enclave_quote_status";
                tracing::error!("{error_msg}");
                tracing::error!("{err}");
                ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
            })?;

            let isv_enclave_quote_status = String::from_utf8(isv_enclave_quote_status.clone())
                .map_err(|err| {
                    let error_msg = "Failed to decode report calldata isv_enclave_quote_status";
                    tracing::error!("{error_msg}");
                    tracing::error!("{err}");
                    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
                })?;

            let encoded_quote_body =
                String::from_utf8(encoded_quote_body.clone()).map_err(|err| {
                    let error_msg = "Failed to decode report calldata isv_enclave_quote_status";
                    tracing::error!("{error_msg}");
                    tracing::error!("{err}");
                    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
                })?;

            let advisory_ids: Vec<String> =
                serde_json::from_slice(advisory_ids).map_err(|err| {
                    let error_msg = "Failed to decode report calldata advisory_ids";
                    tracing::error!("{error_msg}");
                    tracing::error!("{err}");
                    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
                })?;

            AttestationReport {
                id,
                timestamp,
                version,
                epidPseudonym: epid_pseudonym,
                advisoryURL: advisory_url,
                advisoryIDs: advisory_ids,
                isvEnclaveQuoteStatus: isv_enclave_quote_status,
                isvEnclaveQuoteBody: encoded_quote_body,
            }
        }
        _ => {
            let error_msg = "Invalid report calldata";
            tracing::error!("{error_msg}");
            let err = ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg);
            return Err(err);
        }
    };
    Ok(report_data)
}
