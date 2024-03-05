pub mod keygen;
pub mod register_key;
pub mod withdrawal_credentials;

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
    #[command(about = "Generates BLS keyshares to be used for registering a new validator")]
    Keygen {
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
    #[command(about = "Register a validator into PufferProtocol")]
    RegisterKey {
        #[arg(long = "private-key")]
        private_key: String,
        #[arg(long = "rpc-url")]
        rpc_url: String,
        #[arg(long = "puffer-oracle-address")]
        puffer_oracle_address: String,
        #[arg(long = "puffer-protocol-address")]
        puffer_protocol_address: String,
        #[arg(long = "validator-ticket-address")]
        validator_ticket_address: String,
        #[arg(long = "module-name")]
        module_name: String,
        #[arg(long = "number-of-days")]
        number_of_days: u64,
        #[arg(long = "input-file")]
        input_file: PathBuf,
    },
    #[command(about = "Register a validator into PufferProtocol")]
    WithdrawalCredentials {
        #[arg(long = "rpc-url")]
        rpc_url: String,
        #[arg(long = "puffer-protocol-address")]
        puffer_protocol_address: String,
        #[arg(long = "module-address")]
        module_address: String,
    },
}

impl ValidatorCommand {
    pub async fn execute(self) -> AppResult<i32> {
        match self {
            Self::ListKeys { .. } => {
                println!("TODO");
            }
            Self::Keygen {
                guardian_pubkeys,
                guardian_threshold,
                withdrawal_credentials,
                fork_version,
                enclave_url,
                password_file,
                output_file,
            } => {
                keygen::keygen_from_cmd(
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
            Self::RegisterKey {
                private_key,
                rpc_url,
                puffer_oracle_address,
                puffer_protocol_address,
                validator_ticket_address,
                module_name,
                number_of_days,
                input_file,
            } => {
                register_key::register_validator_key(
                    &private_key,
                    &rpc_url,
                    &puffer_oracle_address,
                    &puffer_protocol_address,
                    &validator_ticket_address,
                    &module_name,
                    number_of_days,
                    input_file.as_path(),
                )
                .await?;
            }
            Self::WithdrawalCredentials {
                rpc_url,
                puffer_protocol_address,
                module_address,
            } => {
                withdrawal_credentials::get_withdrawal_credentials(
                    &rpc_url,
                    &puffer_protocol_address,
                    &module_address,
                )
                .await?;
            }
        }
        Ok(0)
    }
}
