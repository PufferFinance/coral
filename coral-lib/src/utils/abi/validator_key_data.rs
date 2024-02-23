use ethers::{abi, prelude::*};

use axum::http::StatusCode;
use puffer_pool_contracts::withdrawal_pool::ValidatorKeyData;

use crate::error::{AppServerResult, ServerErrorResponse};

pub fn from_calldata(tx_data: &[u8]) -> AppServerResult<(ValidatorKeyData, Vec<u8>, U256)> {
    let calldata_tokens = abi::decode(
        &[
            abi::ParamType::Tuple(vec![
                abi::ParamType::Bytes,
                abi::ParamType::Bytes,
                abi::ParamType::FixedBytes(32),
                abi::ParamType::Array(Box::new(abi::ParamType::Bytes)),
                abi::ParamType::Bytes,
                abi::ParamType::Bytes,
            ]),
            abi::ParamType::FixedBytes(32),
            abi::ParamType::Uint(256),
        ],
        tx_data,
    )
    .map_err(|err| {
        let error_msg = "Failed to parse register validator calldata";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;

    let (key_data, module_name, number_of_months) = match calldata_tokens.as_slice() {
        [abi::Token::Tuple(data), abi::Token::FixedBytes(module_name), abi::Token::Uint(number_of_months), ..] =>
        {
            if data.len() != 6 {
                let error_msg = "ValidatorKeyData parameters incorrect";
                tracing::error!("{error_msg}");
                let err =
                    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg);
                return Err(err);
            }

            let key_data: ValidatorKeyData = match data.as_slice() {
                [abi::Token::Bytes(bls_pub_key), abi::Token::Bytes(signature), abi::Token::FixedBytes(deposit_data_root), abi::Token::Array(bls_encrypted_priv_key_shares), abi::Token::Bytes(bls_pub_key_set), abi::Token::Bytes(rave_evidence)] =>
                {
                    let bls_encrypted_priv_key_shares = bls_encrypted_priv_key_shares
                        .iter()
                        .filter_map(|item| match item {
                            abi::Token::Bytes(b) => Some(b.clone().into()),
                            _ => None,
                        })
                        .collect();

                    let deposit_data_root: [u8; 32] =
                        deposit_data_root.clone().try_into().map_err(|_| {
                            let error_msg = "Failed to parse calldata";
                            tracing::error!("{error_msg}");
                            ServerErrorResponse::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                1000,
                                error_msg,
                            )
                        })?;

                    ValidatorKeyData {
                        bls_pub_key: bls_pub_key.clone().into(),
                        signature: signature.clone().into(),
                        deposit_data_root,
                        bls_encrypted_priv_key_shares,
                        bls_pub_key_set: bls_pub_key_set.clone().into(),
                        rave_evidence: rave_evidence.clone().into(),
                    }
                }
                _ => {
                    let error_msg = "Invalid calldata";
                    tracing::error!("{error_msg}");
                    let err = ServerErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        1000,
                        error_msg,
                    );
                    return Err(err);
                }
            };

            (key_data, module_name.clone(), *number_of_months)
        }
        _ => {
            let error_msg = "Invalid register validator calldata";
            tracing::error!("{error_msg}");
            let err = ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg);
            return Err(err);
        }
    };
    Ok((key_data, module_name, number_of_months))
}
