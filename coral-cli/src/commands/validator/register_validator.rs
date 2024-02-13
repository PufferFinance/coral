use std::io::Write;
use std::path::Path;

use axum::http::StatusCode;

use colored::*;

use ecies::PublicKey as EthPublicKey;

use hex::ToHex;
use serde::{Deserialize, Serialize};

use puffersecuresigner::client::traits::ValidatorClientTrait;
use puffersecuresigner::client::{generate_bls_keystore_handler, ClientBuilder};
use puffersecuresigner::enclave::types::AttestFreshBlsKeyPayload;

use coral_lib::error::ServerErrorResponse;
use coral_lib::error::{AppError, AppErrorKind, AppResult};
use coral_lib::strip_0x_prefix;
use coral_lib::structs::eth_types::WithdrawalCredentials;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForkVersionInfo {
    pub current_version: String,
    pub previous_version: String,
    pub genesis_version: String,
    pub genesis_validators_root: String,
    pub epoch: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterValidatorInput {
    pub guardian_pubkeys: Vec<String>,
    pub guardian_threshold: u64,
    pub withdrawal_credentials: String,
    pub fork_version: String,
    pub output_file: String,
    pub enclave_url: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterValidatorOutput {
    pub guardian_threshold: u64,

    pub bls_pub_key_set: String,
    pub bls_pub_key: String,
    pub signature: String,
    pub deposit_data_root: String,
    pub bls_enc_priv_key_shares: Vec<String>,
    pub intel_sig: String,
    pub intel_report: String,
    pub intel_x509: String,
    pub guardian_pubkeys: Vec<String>,
    pub withdrawal_credentials: String,
    pub fork_version: String,
}

pub async fn register_validator_from_cmd(
    guardian_pubkeys: String,
    guardian_threshold: u64,
    withdrawal_credentials: String,
    fork_version: String,
    enclave_url: Option<String>,
    password_file: Option<String>,
    output_file: String,
) -> AppResult<i32> {
    let guardian_pubkeys: Vec<String> =
        guardian_pubkeys.split(',').map(|s| s.to_string()).collect();

    let password = match password_file {
        None => None,
        Some(path) => {
            let password = std::fs::read_to_string(path).map_err(|err| {
                let error_msg = "Failed to read password file";
                eprintln!("{}", error_msg.red());
                err
            })?;
            Some(password.trim().to_string())
        }
    };

    let input_data = RegisterValidatorInput {
        guardian_pubkeys,
        guardian_threshold,
        withdrawal_credentials,
        fork_version,
        enclave_url,
        password,
        output_file,
    };

    register_validator(&input_data).await
}

pub async fn register_validator_from_file(input_file: &Path) -> AppResult<i32> {
    let input_string = std::fs::read_to_string(input_file)?;
    let input_data: RegisterValidatorInput = serde_json::from_str(&input_string)?;

    register_validator(&input_data).await
}
pub async fn register_validator(input_data: &RegisterValidatorInput) -> AppResult<i32> {
    let mut guardian_pubkeys = Vec::with_capacity(input_data.guardian_pubkeys.len());
    for key in input_data.guardian_pubkeys.iter() {
        let key = strip_0x_prefix(key);
        let key_hex = hex::decode(key).map_err(|err| {
            let error_msg = "Failed to parse guardian pubkey";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
        })?;

        let pubkey = EthPublicKey::parse_slice(key_hex.as_slice(), None).map_err(|err| {
            let error_msg = "Failed to parse guardian pubkey";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
        })?;
        guardian_pubkeys.push(pubkey);
    }

    let withdrawal_credentials = strip_0x_prefix(&input_data.withdrawal_credentials);
    let withdrawal_credentials: WithdrawalCredentials = hex::decode(withdrawal_credentials)
        .map_err(|err| {
            let error_msg = "Failed to parse withdrawal_credentials";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
        })?
        .try_into()
        .map_err(|_| {
            let error_msg = "Failed to parse withdrawal_credentials";
            tracing::error!("{error_msg}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
        })?;

    let genesis_fork_version = strip_0x_prefix(&input_data.fork_version);
    let genesis_fork_version = hex::decode(genesis_fork_version)
        .map_err(|err| {
            let error_msg = "Failed to parse fork_version";
            tracing::error!("{error_msg}");
            tracing::error!("{err}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
        })?
        .try_into()
        .map_err(|_| {
            let error_msg = "Failed to parse genesis_fork_version";
            tracing::error!("{error_msg}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
        })?;

    let enclave_enabled = input_data.enclave_url.is_some();

    let enclave_payload = AttestFreshBlsKeyPayload {
        guardian_pubkeys,
        withdrawal_credentials,
        threshold: input_data.guardian_threshold as usize,
        fork_version: genesis_fork_version,
        do_remote_attestation: enclave_enabled,
    };

    let bls_keygen_payload = if enclave_enabled {
        let enclave_url = input_data.enclave_url.as_ref().unwrap();
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

        // enclave

        validator_enclave_client
            .attest_fresh_bls_key(&enclave_payload)
            .await
            .map_err(|err| {
                tracing::error!("Failed to attest_fresh_bls_key");
                tracing::error!("{err}");
                ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1000,
                    "Failed to attest_fresh_bls_key",
                )
            })?
    } else {
        // no enclave
        match input_data.password.as_ref() {
            None => {
                let err =
                    AppError::new(AppErrorKind::ParseError, "No password provided".to_string());
                return Err(err);
            }
            Some(password) => {
                generate_bls_keystore_handler(enclave_payload, password).map_err(|err| {
                    tracing::error!("Failed to attest_fresh_bls_key");
                    tracing::error!("{err}");
                    ServerErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        1000,
                        "Failed to attest_fresh_bls_key",
                    )
                })?
            }
        }
    };

    let registraton_payload = RegisterValidatorOutput {
        guardian_threshold: input_data.guardian_threshold,

        bls_pub_key_set: bls_keygen_payload.bls_pub_key_set,
        bls_pub_key: bls_keygen_payload.bls_pub_key,
        signature: bls_keygen_payload.signature,
        deposit_data_root: bls_keygen_payload.deposit_data_root,
        bls_enc_priv_key_shares: bls_keygen_payload.bls_enc_priv_key_shares,
        intel_sig: bls_keygen_payload.intel_sig,
        intel_report: bls_keygen_payload.intel_report,
        intel_x509: bls_keygen_payload.intel_x509,
        guardian_pubkeys: bls_keygen_payload.guardian_eth_pub_keys,
        withdrawal_credentials: bls_keygen_payload.withdrawal_credentials,
        fork_version: genesis_fork_version.encode_hex(),
    };

    let json_string_pretty = serde_json::to_string_pretty(&registraton_payload)?;

    println!("{}", json_string_pretty);

    {
        let mut file = std::fs::File::create(&input_data.output_file)?;
        file.write_all(json_string_pretty.as_bytes())?;
    }

    Ok(0)
}
