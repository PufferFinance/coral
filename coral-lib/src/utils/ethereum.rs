use std::sync::Arc;

use axum::http::StatusCode;
use ethers::prelude::{
    types::{Address, Block, BlockId, BlockNumber, H256, TransactionReceipt, U256},
    Provider, ProviderError, SignerMiddleware,
};
use url::Url;

use crate::error::{AppServerResult, ServerErrorCode, ServerErrorResponse};

pub fn get_provider(rpc_url: &str) -> AppServerResult<Provider<Http>> {
    let url = Url::parse(rpc_url).map_err(|err| {
        let error_msg = "Invalid RPC URL";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            ServerErrorCode::HttpUrlError,
            err.to_string(),
        )
    })?;
    let transport = Http::new(url);
    let provider = Provider::new(transport);
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
        let error_msg = "Failed to retrieve Chain ID";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::EvmFetchChainIdError,
            err.to_string(),
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
            let error_msg = "Failed to fetch block";
            tracing::error!("{error_msg}: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::EvmFetchBlockError,
                err.to_string(),
            )
        })?
        .ok_or_else(|| {
            let error_msg = "Block does not exist";
            tracing::error!("{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::EvmFetchBlockError,
                error_msg.to_string(),
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
            let error_msg = "Failed to get balance of address";
            tracing::error!("{error_msg}: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::EvmGetBalanceError,
                err.to_string(),
            )
        })?;
    Ok(balance)
}

pub fn get_transaction_receipt(
    tx: Result<Option<TransactionReceipt>, ProviderError>,
) -> AppServerResult<TransactionReceipt> {
    let tx = tx
        .map_err(|err| {
            let error_msg = "Failed to wait for pending transaction";
            tracing::error!("{error_msg}: {err}");
            ServerErrorResponse::new(
                StatusCode::BAD_REQUEST,
                ServerErrorCode::EvmWaitForTransactionError,
                err.to_string(),
            )
        })?
        .ok_or_else(|| {
            let error_msg = "No transaction receipt";
            tracing::error!("{error_msg}");
            ServerErrorResponse::new(
                StatusCode::BAD_REQUEST,
                ServerErrorCode::EvmMissingTransactionReceipt,
                error_msg.to_string(),
            )
        })?;
    Ok(tx)
}
