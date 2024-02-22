use axum::http::StatusCode;

use puffersecuresigner::client::traits::ValidatorClientTrait;
use puffersecuresigner::client::ClientBuilder;

use coral_lib::error::ServerErrorResponse;
use coral_lib::error::{AppError, AppErrorKind, AppResult};
use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Keystore {
    pub pubkey: String,
}

pub async fn list_keys(
    disable_enclave: bool,
    keystore_path: Option<String>,
    enclave_url: Option<String>,
) -> AppResult<i32> {
    if disable_enclave {
        let keystore_path = match keystore_path {
            Some(path) => path,
            None => {
                return Err(AppError::new(
                    AppErrorKind::ParseError,
                    "keystore-path is required when disable-enclave is set".to_string(),
                ));
            }
        };

        let mut dirlist: Vec<std::fs::DirEntry> = std::fs::read_dir(keystore_path)?
            .filter_map(|entry| entry.ok())
            .collect();
        dirlist.sort_by_key(|dir| dir.path());
        for (i, entry) in dirlist.iter().enumerate() {
            let file_bytes = std::fs::read(entry.path())?;
            let keystore: Keystore = serde_json::from_slice(&file_bytes)?;
            println!("{i}: {}", keystore.pubkey);
        }

        Ok(0)
    } else {
        let enclave_url = match enclave_url {
            Some(url) => url,
            None => {
                return Err(AppError::new(
                    AppErrorKind::ParseError,
                    "enclave-url is required when disable-enclave is not set".to_string(),
                ));
            }
        };

        println!("================");
        println!("Enclave URL: '{}'", enclave_url);
        println!("================");

        println!("Running enclave health check...");
        let enclave_client = ClientBuilder::new()
            .validator_url(enclave_url.to_string())
            .build();
        let health_status = enclave_client.validator.health().await;

        let validator_enclave_client = enclave_client.validator;

        if !health_status {
            let err = AppError::new(
                AppErrorKind::EnclaveError,
                "Health check failed".to_string(),
            );
            return Err(err);
        }

        println!("Calling enclave...");

        let keys_result = validator_enclave_client
            .list_bls_keys()
            .await
            .map_err(|err| {
                let error_msg = "Failed to list_bls_keys";
                tracing::error!("{error_msg}");
                tracing::error!("{err}");
                ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
            })?;

        for (i, key) in keys_result.data.iter().enumerate() {
            println!("{i}: {}", key.pubkey);
        }
        Ok(0)
    }
}
