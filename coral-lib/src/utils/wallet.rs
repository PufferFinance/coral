use std::path;

use axum::http::StatusCode;
use ethers::prelude::*;

use crate::{
    error::{AppServerResult, ServerErrorCode, ServerErrorResponse},
    strip_0x_prefix,
};

pub fn read_key_from_file<P: AsRef<path::Path>>(file_path: P) -> std::io::Result<String> {
    let key_content = std::fs::read_to_string(file_path)?;
    let key_content = strip_0x_prefix(key_content.trim()).to_string();
    Ok(key_content)
}

pub fn private_key_file_to_wallet<P: AsRef<path::Path>>(
    private_key_path: P,
) -> AppServerResult<LocalWallet> {
    let private_key = read_key_from_file(private_key_path).map_err(|err| {
        let error_msg = "Failed to read private key file";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            err.to_string(),
        )
    })?;
    let wallet: LocalWallet = private_key.parse().map_err(|err: WalletError| {
        let error_msg = "Invalid private key";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            err.to_string(),
        )
    })?;
    Ok(wallet)
}

/// Generate a random wallet.
/// Mostly used for calling view and pure functions
pub fn generate_random_wallet() -> LocalWallet {
    LocalWallet::new(&mut rand::thread_rng())
}
