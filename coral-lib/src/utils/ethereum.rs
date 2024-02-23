use std::sync::Arc;

use axum::http::StatusCode;
use ethers::prelude::*;

use crate::error::{AppServerResult, ServerErrorResponse};

pub fn get_provider(rpc_url: &str) -> AppServerResult<Provider<Http>> {
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| {
        tracing::error!("Invalid RPC URL");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, "Invalid RPC URL")
    })?;
    Ok(provider)
}

pub fn get_client<J, E>(
    provider: Provider<J>,
    wallet: LocalWallet,
    chain_id: u64,
) -> Arc<SignerMiddleware<Provider<J>, LocalWallet>>
where
    J: JsonRpcClient<Error = E>,
{
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    Arc::new(client)
}

pub async fn get_chain_id<J, E>(provider: &Provider<J>) -> AppServerResult<U256>
where
    J: JsonRpcClient<Error = E>,
{
    let chain_id = provider.get_chainid().await.map_err(|err| {
        tracing::error!("Failed to retrieve Chain ID");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to retrieve Chain ID",
        )
    })?;
    Ok(chain_id)
}

pub async fn get_block<J, E>(
    provider: &Provider<J>,
    block_number: BlockNumber,
) -> AppServerResult<Block<H256>>
where
    J: JsonRpcClient<Error = E>,
{
    let block = provider
        .get_block(block_number)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch block");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to fetch block",
            )
        })?
        .ok_or_else(|| {
            tracing::error!("Block does not exist");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Block does not exist",
            )
        })?;
    Ok(block)
}

pub async fn get_balance<J, E>(
    provider: &Provider<J>,
    address: Address,
    block_id: BlockId,
) -> AppServerResult<U256>
where
    J: JsonRpcClient<Error = E>,
{
    let balance = provider
        .get_balance(address, Some(block_id))
        .await
        .map_err(|err| {
            tracing::error!("Failed to get balance of address");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to get balance of address",
            )
        })?;
    Ok(balance)
}
