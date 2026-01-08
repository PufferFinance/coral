use coral_lib::error::{AppError, AppErrorKind, AppResult};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Keystore {
    pub pubkey: String,
}

/// Default keystore path for BLS keys
const DEFAULT_KEYSTORE_PATH: &str = "./etc/keys/bls_keys";

pub async fn list_keys(keystore_path: Option<String>) -> AppResult<i32> {
    let keystore_path = keystore_path.unwrap_or_else(|| DEFAULT_KEYSTORE_PATH.to_string());

    if !std::path::Path::new(&keystore_path).exists() {
        return Err(AppError::new(
            AppErrorKind::ParseError,
            format!("Keystore path does not exist: {}", keystore_path),
        ));
    }

    let mut dirlist: Vec<std::fs::DirEntry> = std::fs::read_dir(&keystore_path)?
        .filter_map(|entry| entry.ok())
        .collect();
    dirlist.sort_by_key(|dir| dir.path());

    if dirlist.is_empty() {
        println!("No keys found in {}", keystore_path);
        return Ok(0);
    }

    for (i, entry) in dirlist.iter().enumerate() {
        let file_bytes = std::fs::read(entry.path())?;
        let keystore: Keystore = serde_json::from_slice(&file_bytes)?;
        println!("{i}: {}", keystore.pubkey);
    }

    Ok(0)
}
