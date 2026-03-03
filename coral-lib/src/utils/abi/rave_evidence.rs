use ethers::abi;

use axum::http::StatusCode;

use crate::error::{AppServerResult, ServerErrorCode, ServerErrorResponse};

#[derive(Clone, Debug)]
pub struct AbiDecodedSessionEvidence {
    pub session_id: Vec<u8>,
    pub attestation_signature: Vec<u8>,
    pub session_public_key: Vec<u8>,
}

pub fn to_calldata(
    session_id: &[u8],
    attestation_signature: &[u8],
    session_public_key: &[u8],
) -> AppServerResult<abi::Bytes> {
    let evidence = abi::encode(&[
        abi::Token::Bytes(session_id.into()),
        abi::Token::Bytes(attestation_signature.into()),
        abi::Token::Bytes(session_public_key.to_vec()),
    ]);
    Ok(evidence)
}

pub fn from_calldata(data: &[u8]) -> AppServerResult<AbiDecodedSessionEvidence> {
    let calldata_tokens = abi::decode(
        &[
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
            abi::ParamType::Bytes,
        ],
        data,
    )
    .map_err(|err| {
        let error_msg = "Failed to parse session evidence calldata";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::EnclaveInvalidRaveCalldata,
            err.to_string(),
        )
    })?;

    let evidence = match calldata_tokens.as_slice() {
        [abi::Token::Bytes(session_id), abi::Token::Bytes(attestation_signature), abi::Token::Bytes(session_public_key), ..] => {
            AbiDecodedSessionEvidence {
                session_id: session_id.clone(),
                attestation_signature: attestation_signature.clone(),
                session_public_key: session_public_key.clone(),
            }
        }
        _ => {
            let error_msg = "Invalid session evidence calldata";
            tracing::error!("{error_msg}");
            let err = ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::EnclaveInvalidRaveCalldata,
                error_msg.to_string(),
            );
            return Err(err);
        }
    };
    Ok(evidence)
}
