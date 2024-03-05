use ethers::abi;

use axum::http::StatusCode;

use crate::error::{AppServerResult, ServerErrorCode, ServerErrorResponse};

#[derive(Clone, Debug)]
pub struct AbiDecodedRaveData {
    pub enclave_report: Vec<u8>,
    pub enclave_sig: Vec<u8>,
    pub enclave_x509: Vec<u8>,
}

pub fn to_calldata(
    enclave_sig: &[u8],
    enclave_report: &[u8],
    enclave_x509: &[u8],
) -> AppServerResult<abi::Bytes> {
    let rave_evidence = abi::encode(&[
        abi::Token::Bytes(enclave_sig.into()),
        abi::Token::Bytes(enclave_report.into()),
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
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::EnclaveInvalidRaveCalldata,
            err.to_string(),
        )
    })?;

    let rave_data = match calldata_tokens.as_slice() {
        [abi::Token::Bytes(enclave_sig), abi::Token::Bytes(enclave_report), abi::Token::Bytes(enclave_x509), ..] => {
            AbiDecodedRaveData {
                enclave_sig: enclave_sig.clone(),
                enclave_report: enclave_report.clone(),
                enclave_x509: enclave_x509.clone(),
            }
        }
        _ => {
            let error_msg = "Invalid RAVE calldata";
            tracing::error!("{error_msg}");
            let err = ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::EnclaveInvalidRaveCalldata,
                error_msg.to_string(),
            );
            return Err(err);
        }
    };
    Ok(rave_data)
}
