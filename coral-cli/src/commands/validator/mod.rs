pub mod register_validator;

use std::path::PathBuf;

use clap::Subcommand;

use coral_lib::error::AppResult;

#[derive(Clone, Debug, Subcommand)]
pub enum ValidatorCommand {
    #[command(about = "List BLS keys")]
    ListKeys {
        #[arg(long = "disable-enclave")]
        disable_enclave: bool,
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
        #[arg(long = "password")]
        password: Option<String>,
        #[arg(long = "output-file")]
        output_file: String,
    },
    #[command(about = "Register a validator into Puffer's Pool")]
    RegisterWithFile {
        #[arg(long = "input-file")]
        input_file: PathBuf,
    },
}

impl ValidatorCommand {
    pub async fn execute(self) -> AppResult<i32> {
        match self {
            Self::ListKeys { .. } => {
                println!("TODO");
            }
            Self::Register {
                guardian_pubkeys,
                guardian_threshold,
                withdrawal_credentials,
                fork_version,
                enclave_url,
                password,
                output_file,
            } => {
                register_validator::register_validator_from_cmd(
                    guardian_pubkeys,
                    guardian_threshold,
                    withdrawal_credentials,
                    fork_version,
                    enclave_url,
                    password,
                    output_file,
                )
                .await?;
            }
            Self::RegisterWithFile { input_file } => {
                register_validator::register_validator_from_file(input_file.as_path()).await?;
            }
        }
        Ok(0)
    }
}
