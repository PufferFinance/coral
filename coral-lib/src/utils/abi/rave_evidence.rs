use ethers::abi;
use ethers::utils::hex::ToHex;

use axum::http::StatusCode;
use puffersecuresigner::enclave::types::KeyGenResponse;
use puffersecuresigner::io::remote_attestation::AttestationReport;

use crate::error::{AppServerResult, ServerErrorResponse};
use crate::utils;

use super::{SIGNING_EXP, SIGNING_MOD};

#[derive(Clone, Debug)]
pub struct AbiDecodedRaveData {
    pub enclave_report: Vec<u8>,
    pub enclave_sig: Vec<u8>,
    pub enclave_x509: Vec<u8>,
}

pub fn to_json(
    report: &KeyGenResponse,
    attestation_report: &AttestationReport,
) -> AppServerResult<String> {
    let evidence = &report.evidence;
    let signature = utils::encoding::base64_decode_to_bytes(&evidence.signed_report)?;

    let (leaf_x509, _) = utils::ssl::extract_x509(report.evidence.signing_cert.as_bytes())?;
    let leaf_x509_der = leaf_x509.to_der().map_err(|err| {
        let msg = "Failed to convert x509 to DER format";
        tracing::error!(msg);
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, msg)
    })?;

    let quote_body = attestation_report.deserialize_quote_body().unwrap();

    let json_data = serde_json::json!({
        "evidence": report.evidence,
        "signature": signature.encode_hex::<String>(),
        "leaf_x509": leaf_x509_der.encode_hex::<String>(),
        "mr_enclave": quote_body.MRENCLAVE,
        "mr_signer": quote_body.MRSIGNER,
        "signing_mod": SIGNING_MOD,
        "signing_exp": SIGNING_EXP,
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

pub fn to_calldata(
    enclave_report: &[u8],
    enclave_sig: &[u8],
    enclave_x509: &[u8],
) -> AppServerResult<abi::Bytes> {
    let rave_evidence = abi::encode(&[
        abi::Token::Bytes(enclave_report.into()),
        abi::Token::Bytes(enclave_sig.into()),
        abi::Token::Bytes(enclave_x509.to_vec()),
    ]);
    Ok(rave_evidence)
}

pub fn from_calldata(data: &[u8]) -> AppServerResult<AbiDecodedRaveData> {
    let calldata_tokens = abi::decode(
        &[
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
        ],
        data,
    )
    .map_err(|err| {
        let error_msg = "Failed to parse RAVE calldata";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;

    // eprintln!("RAVE_calldata: {:?}", calldata_tokens);

    let rave_data = match calldata_tokens.as_slice() {
        [abi::Token::Bytes(enclave_report), abi::Token::Bytes(enclave_sig), abi::Token::Bytes(enclave_x509), ..] => {
            AbiDecodedRaveData {
                enclave_report: enclave_report.clone(),
                enclave_sig: enclave_sig.clone(),
                enclave_x509: enclave_x509.clone(),
            }
        }
        _ => {
            let error_msg = "Invalid RAVE calldata";
            tracing::error!("{error_msg}");
            let err = ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg);
            return Err(err);
        }
    };
    Ok(rave_data)
}
