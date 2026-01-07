use std::io::Write;

use axum::http::StatusCode;

use colored::Colorize;

use coral_lib::keygen::{generate_bls_keystore_handler, AttestFreshBlsKeyPayload};
use coral_lib::utils::parse::parse_module_name;

use hex::ToHex;
use serde::{Deserialize, Serialize};

use coral_lib::error::AppResult;
use coral_lib::error::{ServerErrorCode, ServerErrorResponse};
use coral_lib::strip_0x_prefix;
use coral_lib::structs::eth_types::WithdrawalCredentials;

use crate::APP_VERSION;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlsKeygenInput {
    pub module_name: String,
    pub withdrawal_credentials: String,
    pub fork_version: String,
    pub output_file: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlsKeygenOutput {
    pub version: String,
    pub module_name: String,
    pub withdrawal_credentials: String,
    pub fork_version: String,
    pub signature: String,
    pub deposit_data_root: String,
    pub bls_pub_key: String,
}

#[derive(Clone, Debug)]
pub struct KeygenCmdInput {
    pub module_name: String,
    pub withdrawal_credentials: String,
    pub fork_version: String,
    pub password_file: String,
    pub output_file: String,
}

pub async fn keygen_from_cmd(data: KeygenCmdInput) -> AppResult<i32> {
    let KeygenCmdInput {
        module_name,
        withdrawal_credentials,
        fork_version,
        password_file,
        output_file,
    } = data;

    let password = std::fs::read_to_string(&password_file).inspect_err(|err| {
        let error_msg = "Failed to read password file";
        eprintln!("{}", error_msg.red());
        eprintln!("Error details: {}", err);
    })?;
    let password = password.trim().to_string();

    let input_data = BlsKeygenInput {
        module_name,
        withdrawal_credentials,
        fork_version,
        password,
        output_file,
    };

    register_validator(&input_data).await
}

pub async fn register_validator(input_data: &BlsKeygenInput) -> AppResult<i32> {
    let module_name = parse_module_name(&input_data.module_name)?;

    let withdrawal_credentials = strip_0x_prefix(&input_data.withdrawal_credentials);
    let withdrawal_credentials: WithdrawalCredentials = hex::decode(withdrawal_credentials)
        .map_err(|err| {
            let error_msg = format!("Failed to parse withdrawal_credentials: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::ParseError,
                error_msg,
            )
        })?
        .try_into()
        .map_err(|_| {
            let error_msg = "Failed to parse withdrawal_credentials".to_owned();
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::ParseError,
                error_msg,
            )
        })?;

    let genesis_fork_version = strip_0x_prefix(&input_data.fork_version);
    let genesis_fork_version = hex::decode(genesis_fork_version)
        .map_err(|err| {
            let error_msg = format!("Failed to parse fork_version: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::ParseError,
                error_msg,
            )
        })?
        .try_into()
        .map_err(|_| {
            let error_msg = "Failed to parse genesis_fork_version".to_owned();
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::ParseError,
                error_msg.to_string(),
            )
        })?;

    if input_data.password.len() < 8 {
        let error_msg = "Password must be at least 8 characters";
        let err = ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            error_msg.to_string(),
        );
        return Err(err.into());
    }

    let keygen_payload = AttestFreshBlsKeyPayload {
        withdrawal_credentials,
        fork_version: genesis_fork_version,
    };

    let bls_keygen_payload = generate_bls_keystore_handler(keygen_payload, &input_data.password)
        .map_err(|err| {
            let error_msg = format!("Failed to generate BLS keystore: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ServerErrorCode::ParseError,
                error_msg,
            )
        })?;

    let registration_payload = BlsKeygenOutput {
        version: APP_VERSION.to_string(),
        module_name: module_name.encode_hex(),
        withdrawal_credentials: hex::encode(withdrawal_credentials),
        fork_version: genesis_fork_version.encode_hex(),

        signature: bls_keygen_payload.signature,
        deposit_data_root: bls_keygen_payload.deposit_data_root,
        bls_pub_key: bls_keygen_payload.bls_pub_key,
    };

    let json_string_pretty = serde_json::to_string_pretty(&registration_payload)?;

    println!("{}", json_string_pretty);
    {
        let mut file = std::fs::File::create(&input_data.output_file)?;
        file.write_all(json_string_pretty.as_bytes())?;
    }

    std::fs::rename(
        format!("etc/keys/bls_keys/{}", registration_payload.bls_pub_key),
        format!(
            "etc/keys/bls_keys/{}.json",
            registration_payload.bls_pub_key
        ),
    )?;

    Ok(0)
}
