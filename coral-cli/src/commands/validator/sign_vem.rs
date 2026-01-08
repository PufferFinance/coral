use coral_lib::error::{AppError, AppErrorKind, AppResult};

/// Sign voluntary exit message command.
/// Note: This command is currently not supported
/// The exiting will be supported on chain using EIP-7002.
#[allow(clippy::too_many_arguments)]
pub async fn sign_vem_from_cmd(
    _enclave_url: String,
    _bls_pubkey: String,
    _beacon_index: u64,
    _fork_current_version: String,
    _fork_previous_version: String,
    _epoch: u64,
    _genesis_validators_root: String,
    _output_file: String,
) -> AppResult<i32> {
    Err(AppError::new(
        AppErrorKind::NotSupportedError,
        "SignVoluntaryExit command is not supported. \
         The exiting will be supported on chain using EIP-7002."
            .to_string(),
    ))
}
