use std::sync::Arc;

use axum::http::StatusCode;

use ethers::prelude::*;
use ethers::types::Address;
use ethers::types::Log;
use ethers::utils::keccak256;

use puffer_pool_contracts::puffer_protocol::PufferProtocol;

use crate::error::{AppServerResult, ServerErrorResponse};

pub async fn find_register_validator_event_by_index<J, E>(
    client: Arc<SignerMiddleware<Provider<J>, LocalWallet>>,
    puffer_protocol_address: Address,
    validator_index: U256,
) -> AppServerResult<Log>
where
    J: JsonRpcClient<Error = E>,
{
    let contract: PufferProtocol<_> = PufferProtocol::new(puffer_protocol_address, client.clone());
    let filter = contract
        .validator_key_registered_filter()
        .filter
        .address(puffer_protocol_address)
        .from_block(0);

    let logs = client.get_logs(&filter).await.map_err(|err| {
        let error_msg = "Failed to get logs";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;

    for log in logs {
        let log_validator_index = U256::from_big_endian(log.topics[2].as_bytes());
        if log_validator_index == validator_index {
            return Ok(log);
        }
    }

    let err = ServerErrorResponse::new(StatusCode::NOT_FOUND, 1000, "Validator event not found");
    Err(err)
}

pub async fn find_register_validator_event_by_address<J, E>(
    client: Arc<SignerMiddleware<Provider<J>, LocalWallet>>,
    puffer_protocol_address: Address,
    validator_address: Vec<u8>,
) -> AppServerResult<Log>
where
    J: JsonRpcClient<Error = E>,
{
    let contract: PufferProtocol<_> = PufferProtocol::new(puffer_protocol_address, client.clone());
    let filter = contract
        .validator_key_registered_filter()
        .filter
        .address(puffer_protocol_address)
        .from_block(0);

    let logs = client.get_logs(&filter).await.map_err(|err| {
        let error_msg = "Failed to get logs";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;

    let hashed_address = keccak256(validator_address);
    for log in logs {
        let bls_key = log.topics[1];
        if *bls_key.as_fixed_bytes() == hashed_address {
            return Ok(log);
        }
    }

    let err = ServerErrorResponse::new(StatusCode::NOT_FOUND, 1000, "Validator event not found");
    Err(err)
}
