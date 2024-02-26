pub mod list_keys;
pub mod register_validator;
pub mod sign_vem;

use std::path::PathBuf;

use clap::Subcommand;

use coral_lib::error::AppResult;

#[derive(Clone, Debug, Subcommand)]
pub enum ValidatorCommand {
    #[command(about = "List BLS keys")]
    ListKeys {
        #[arg(long = "disable-enclave")]
        disable_enclave: bool,
        #[arg(long = "keystore-path")]
        keystore_path: Option<String>,
        #[arg(long = "enclave-url")]
        enclave_url: Option<String>,
    },
    #[command(about = "Register a validator into Puffer's Pool")]
    Register {
        #[arg(long = "guardian-pubkeys")]
        guardian_pubkeys: String,
        #[arg(long = "guardian-threshold")]
        guardian_threshold: u64,
        #[arg(long = "withdrawal-credentials")]
        withdrawal_credentials: String,
        #[arg(long = "fork-version")]
        fork_version: String,
        #[arg(long = "enclave-url")]
        enclave_url: Option<String>,
        #[arg(long = "password-file")]
        password_file: Option<String>,
        #[arg(long = "output-file")]
        output_file: String,
    },
    #[command(about = "Register a validator into Puffer's Pool")]
    RegisterWithFile {
        #[arg(long = "input-file")]
        input_file: PathBuf,
    },
    SignVoluntaryExit {
        #[arg(long = "bls-public-key")]
        bls_pubkey: String,
        #[arg(long = "beacon-index")]
        beacon_index: u64,
        #[arg(long = "enclave-url")]
        enclave_url: String,
        #[arg(long = "beacon-url")]
        beacon_url: String,
        #[arg(long = "fork-previous-version")]
        fork_previous_version: String,
        #[arg(long = "fork-current-version")]
        fork_current_version: String,
        #[arg(long = "epoch")]
        epoch: u64,
        #[arg(long = "genesis-validators-root")]
        genesis_validators_root: String,
        #[arg(long = "output-file")]
        output_file: String,
    },
}

impl ValidatorCommand {
    pub async fn execute(self) -> AppResult<i32> {
        match self {
            Self::ListKeys {
                disable_enclave,
                keystore_path,
                enclave_url,
            } => {
                list_keys::list_keys(disable_enclave, keystore_path, enclave_url).await?;
            }
            Self::Register {
                guardian_pubkeys,
                guardian_threshold,
                withdrawal_credentials,
                fork_version,
                enclave_url,
                password_file,
                output_file,
            } => {
                register_validator::register_validator_from_cmd(
                    guardian_pubkeys,
                    guardian_threshold,
                    withdrawal_credentials,
                    fork_version,
                    enclave_url,
                    password_file,
                    output_file,
                )
                .await?;
            }
            Self::RegisterWithFile { input_file } => {
                register_validator::register_validator_from_file(input_file.as_path()).await?;
            }
            Self::SignVoluntaryExit {
                enclave_url,
                bls_pubkey,
                beacon_index,
                beacon_url,
                fork_current_version,
                fork_previous_version,
                epoch,
                genesis_validators_root,
                output_file,
            } => {
                sign_vem::sign_vem_from_cmd(
                    enclave_url,
                    bls_pubkey,
                    beacon_index,
                    beacon_url,
                    fork_current_version,
                    fork_previous_version,
                    epoch,
                    genesis_validators_root,
                    output_file,
                )
                .await?;
            }
        }
        Ok(0)
    }
}
