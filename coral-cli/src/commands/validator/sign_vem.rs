use axum::http::StatusCode;
use coral_lib::add_0x_prefix;
use coral_lib::error::{AppError, AppErrorKind, AppResult, ServerErrorResponse};
use coral_lib::structs::eth_types::ForkVersion;
use puffersecuresigner::client::{traits::ValidatorClientTrait, ClientBuilder};
use puffersecuresigner::eth2::eth_types::{Fork, ForkInfo, Root};
use puffersecuresigner::strip_0x_prefix;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExitResponseOutput {
    pub signature: String,
    pub beacon_index: u64,
    pub epoch: u64,
    pub bls_pubkey: String,
}

#[derive(Clone, Debug)]
pub struct SignVoluntaryExitMessageInput {
    pub bls_pubkey: String,
    pub beacon_index: u64,
    pub enclave_url: String,
    pub beacon_url: String,
    pub fork: Fork,
    pub genesis_validators_root: [u8; 32],
    pub output_file: String,
}

#[allow(clippy::too_many_arguments)]
pub async fn sign_vem_from_cmd(
    enclave_url: String,
    bls_pubkey: String,
    beacon_index: u64,
    beacon_url: String,
    fork_current_version: String,
    fork_previous_version: String,
    epoch: u64,
    genesis_validators_root: String,
    output_file: String,
) -> AppResult<i32> {
    let converted_fork_info: ForkInfo = convert_to_fork_formats(
        fork_current_version,
        fork_previous_version,
        epoch,
        genesis_validators_root,
    )
    .unwrap();

    let input_data = SignVoluntaryExitMessageInput {
        bls_pubkey,
        beacon_index,
        enclave_url,
        beacon_url,
        fork: converted_fork_info.fork,
        genesis_validators_root: converted_fork_info.genesis_validators_root,
        output_file,
    };
    sign_voluntary_exit_message(input_data).await
}

pub async fn sign_voluntary_exit_message(
    input_data: SignVoluntaryExitMessageInput,
) -> AppResult<i32> {
    let enclave_url = input_data.enclave_url;

    let enclave_client = ClientBuilder::new()
        .validator_url(enclave_url.to_string())
        .build();

    let validator_enclave_client = enclave_client.validator;

    let health_status = validator_enclave_client.health().await;
    if !health_status {
        let err = AppError::new(
            AppErrorKind::EnclaveError,
            "Enclave health check failed".to_string(),
        );
        return Err(err);
    }

    let bls_public_key = add_0x_prefix(&input_data.bls_pubkey);

    let fork_info = ForkInfo {
        fork: Fork {
            previous_version: input_data.fork.previous_version,
            current_version: input_data.fork.current_version,
            epoch: input_data.fork.epoch,
        },
        genesis_validators_root: input_data.genesis_validators_root,
    };

    let sign_exit_resp = validator_enclave_client
        .sign_voluntary_exit_message(
            bls_public_key,
            fork_info.fork.epoch,
            input_data.beacon_index,
            fork_info.clone(),
        )
        .await
        .map_err(|err| {
            let error_msg = "Failed to sign_voluntary_exit_message";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;

    let epoch = fork_info.clone().fork.epoch;
    let exit_payload = ExitResponseOutput {
        signature: sign_exit_resp.signature,
        beacon_index: input_data.beacon_index,
        epoch,
        bls_pubkey: input_data.bls_pubkey,
    };

    let json_string_pretty = serde_json::to_string_pretty(&exit_payload)?;
    println!("{}", json_string_pretty);

    {
        let mut file = std::fs::File::create(&input_data.output_file)?;
        file.write_all(json_string_pretty.as_bytes())?;
    }

    Ok(0)
}

// Helper to convert from String to Fork formats
fn convert_to_fork_formats(
    fork_current_version: String,
    fork_previous_version: String,
    epoch: u64,
    genesis_validators_root: String,
) -> AppResult<ForkInfo> {
    let previous_version: &str = strip_0x_prefix!(fork_previous_version.as_str());
    let fork_previous_version: ForkVersion = hex::decode(previous_version)
        .map_err(|_err| {
            AppError::new(
                AppErrorKind::DecodeError,
                "fork.previous_version: Failed to decode hex".to_string(),
            )
        })?
        .as_slice()
        .try_into()
        .map_err(|_err| {
            AppError::new(
                AppErrorKind::DecodeError,
                "fork.previous_version: Invalid length".to_string(),
            )
        })?;
    let current_version: &str = strip_0x_prefix!(fork_current_version.as_str());
    let fork_current_version: ForkVersion = hex::decode(current_version)
        .map_err(|_err| {
            AppError::new(
                AppErrorKind::DecodeError,
                "fork.current_version: Failed to decode hex".to_string(),
            )
        })?
        .as_slice()
        .try_into()
        .map_err(|_err| {
            AppError::new(
                AppErrorKind::DecodeError,
                "fork.current_version: Invalid length".to_string(),
            )
        })?;

    let genesis_validators_root: &str = strip_0x_prefix!(genesis_validators_root.as_str());
    let genesis_validators_root: Root = hex::decode(genesis_validators_root)
        .map_err(|_err| {
            AppError::new(
                AppErrorKind::DecodeError,
                "genesis_validators_root: Failed to decode hex".to_string(),
            )
        })?
        .as_slice()
        .try_into()
        .map_err(|_err| {
            AppError::new(
                AppErrorKind::DecodeError,
                "fork.genesis_validators_root: Invalid length".to_string(),
            )
        })?;

    let fork_config = ForkInfo {
        fork: Fork {
            current_version: fork_current_version,
            previous_version: fork_previous_version,
            epoch,
        },
        genesis_validators_root,
    };

    Ok(fork_config)
}
