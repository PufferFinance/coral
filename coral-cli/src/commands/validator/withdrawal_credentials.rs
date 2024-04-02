use ethers::prelude::Address;

use coral_lib::error::{AppError, AppErrorKind, AppResult};
use coral_lib::utils;

use crate::PufferProtocol;

pub async fn get_withdrawal_credentials(
    rpc_url: &str,
    puffer_protocol_address: &str,
    module_address: &str,
) -> AppResult<i32> {
    let puffer_protocol_address_h160: Address = puffer_protocol_address.parse().map_err(|_| {
        AppError::new(
            AppErrorKind::DecodeError,
            format!(
                "Invalid Puffer Protocol address: '{}'",
                puffer_protocol_address
            ),
        )
    })?;

    let module_address_h160: Address = module_address.parse().map_err(|_| {
        AppError::new(
            AppErrorKind::DecodeError,
            format!("Invalid Module address: '{}'", module_address),
        )
    })?;

    let provider = utils::ethereum::get_provider(rpc_url)?;
    let wallet = utils::wallet::generate_random_wallet();
    let chain_id = utils::ethereum::get_chain_id(&provider).await?;
    let client = utils::ethereum::get_client(provider.clone(), wallet.clone(), chain_id.as_u64());

    let puffer_protocol_contract: PufferProtocol<_> =
        PufferProtocol::new(puffer_protocol_address_h160, client.clone());

    let withdrawal_credentials = puffer_protocol_contract
        .get_withdrawal_credentials(module_address_h160)
        .await
        .map_err(|err| AppError::new(AppErrorKind::DecodeError, err.to_string()))?;

    println!("{}", withdrawal_credentials);

    Ok(0)
}
