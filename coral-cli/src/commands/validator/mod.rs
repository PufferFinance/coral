pub mod keygen;
pub mod list_keys;
pub mod register_calldata;
pub mod register_key;
pub mod sign_vem;
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
        #[arg(long = "keystore-path")]
        keystore_path: Option<String>,
        #[arg(long = "enclave-url")]
        enclave_url: Option<String>,
    },
    #[command(about = "Generates BLS keyshares to be used for registering a new validator")]
    Keygen {
        #[arg(long = "guardian-pubkeys")]
        guardian_pubkeys: String,
        #[arg(long = "guardian-threshold")]
        guardian_threshold: u64,
        #[arg(long = "module-name")]
        module_name: String,
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
    #[command(about = "Register a validator into PufferProtocol (for testing only)")]
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
    #[command(about = "Generate calldata for registering a validator (for testing only)")]
    GenerateRegisterCalldata {
        #[arg(long = "rpc-url")]
        rpc_url: String,
        #[arg(long = "puffer-protocol-address")]
        puffer_protocol_address: String,
        #[arg(long = "validator-ticket-address")]
        validator_ticket_address: String,
        #[arg(long = "module-name")]
        module_name: String,
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
    #[command(about = "Fetch withdrawal credentials for a given module")]
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
            Self::ListKeys {
                disable_enclave,
                keystore_path,
                enclave_url,
            } => {
                list_keys::list_keys(disable_enclave, keystore_path, enclave_url).await?;
            }
            Self::Keygen {
                guardian_pubkeys,
                guardian_threshold,
                module_name,
                withdrawal_credentials,
                fork_version,
                enclave_url,
                password_file,
                output_file,
            } => {
                let data = keygen::KeygenCmdInput {
                    guardian_pubkeys,
                    guardian_threshold,
                    module_name,
                    withdrawal_credentials,
                    fork_version,
                    enclave_url,
                    password_file,
                    output_file,
                };
                keygen::keygen_from_cmd(data).await?;
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
            Self::SignVoluntaryExit {
                enclave_url,
                bls_pubkey,
                beacon_index,
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
                    fork_current_version,
                    fork_previous_version,
                    epoch,
                    genesis_validators_root,
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
            Self::GenerateRegisterCalldata {
                rpc_url,
                puffer_protocol_address,
                validator_ticket_address,
                module_name,
                input_file,
            } => {
                register_calldata::generate_register_calldata(
                    &rpc_url,
                    &puffer_protocol_address,
                    &validator_ticket_address,
                    &module_name,
                    input_file.as_path(),
                )
                .await?;
            }
        }
        Ok(0)
    }
}
