use coral_lib::error::{AppError, AppErrorKind, AppResult};
use coral_lib::keygen::simple_bls_keygen::generate_and_save_bls_key;
use std::path::Path;

/// Default output directory for generated BLS keystores
const DEFAULT_OUTPUT_DIR: &str = "./etc/keys/bls_keys";

pub async fn generate_bls_key(
    password_file: String,
    output_dir: Option<String>,
) -> AppResult<i32> {
    // Read password from file
    let password = std::fs::read_to_string(&password_file).map_err(|err| {
        AppError::new(
            AppErrorKind::ReadFileError,
            format!("Failed to read password file '{}': {}", password_file, err),
        )
    })?;
    let password = password.trim();

    // Validate password length
    if password.len() < 8 {
        return Err(AppError::new(
            AppErrorKind::ParseError,
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Use provided output directory or default
    let output_dir = output_dir.unwrap_or_else(|| DEFAULT_OUTPUT_DIR.to_string());
    let output_path = Path::new(&output_dir);

    // Generate key and save keystore
    let (pubkey_hex, keystore_path) =
        generate_and_save_bls_key(output_path, password).map_err(|err| {
            AppError::new(
                AppErrorKind::AppError,
                format!("Failed to generate BLS key: {}", err),
            )
        })?;

    println!("BLS key generated successfully!");
    println!("Public key: {}", pubkey_hex);
    println!("Keystore saved to: {}", keystore_path);

    Ok(0)
}
