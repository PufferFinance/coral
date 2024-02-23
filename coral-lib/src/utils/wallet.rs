use std::fs;
use std::io::prelude::Read;
use std::path;

use axum::http::StatusCode;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use rand_core::OsRng;
use puffersecuresigner::strip_0x_prefix;

use crate::
    error::{AppError, AppErrorKind, AppResult, AppServerResult, ServerErrorResponse}
;

pub fn generate_keystore(password: &str) -> AppServerResult<(Wallet<SigningKey>, String)> {
    let (new_wallet, keystore_uuid) = Wallet::new_keystore(".", &mut OsRng, password, None)
        .map_err(|err| {
            tracing::error!("Failed to generate keystore");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to generate keystore",
            )
        })?;

    // Ethers Keystore API must be generated and stored in a file.
    // So we have to read from it and delete it after
    let keystore_json = {
        let mut file = fs::File::open(&keystore_uuid).map_err(|err| {
            tracing::error!("Failed to read keystore");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to read keystore",
            )
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|err| {
            tracing::error!("Failed to read keystore to string");
            tracing::error!("{err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to read keystore to string",
            )
        })?;

        // remove file after reading it
        let _ = fs::remove_file(&keystore_uuid);
        contents
    };

    Ok((new_wallet, keystore_json))
}

pub fn read_key_from_file<P: AsRef<path::Path>>(file_path: P) -> std::io::Result<String> {
    let key_content = std::fs::read_to_string(file_path)?;
    let key_content: String = strip_0x_prefix!(key_content.trim());
    Ok(key_content)
}

pub fn private_key_file_to_wallet<P: AsRef<path::Path>>(
    private_key_path: P,
) -> AppResult<LocalWallet> {
    let private_key = read_key_from_file(private_key_path)?;
    let wallet: LocalWallet = private_key.parse().map_err(|err| {
        tracing::error!("Invalid private key: '{}'", private_key);
        tracing::error!("{err}");
        AppError::new(AppErrorKind::ParseError, "Invalid private key".to_string())
    })?;
    Ok(wallet)
}

/// Generate a random wallet.
/// Mostly used for calling view and pure functions
pub fn generate_random_wallet() -> LocalWallet {
    LocalWallet::new(&mut rand::thread_rng())
}
