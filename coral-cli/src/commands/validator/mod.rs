pub mod generate_bls_key;
pub mod keygen;
pub mod list_keys;
pub mod sign_vem;
pub mod verify_merkle_tree_rewards;

#[cfg(feature = "dev")]
pub mod register_calldata;
#[cfg(feature = "dev")]
pub mod register_key;

#[cfg(feature = "dev")]
pub mod withdrawal_credentials;

#[cfg(feature = "dev")]
use std::path::PathBuf;

use clap::Subcommand;

use coral_lib::error::AppResult;

#[derive(Clone, Debug, Subcommand)]
pub enum ValidatorCommand {
    #[command(about = "Generate a new BLS key pair and save keystore in a pubkey-named directory")]
    GenerateBlsKey {
        #[arg(long = "password-file")]
        password_file: String,
        #[arg(long = "output-dir")]
        output_dir: Option<String>,
    },
    #[command(about = "List BLS keys")]
    ListKeys {
        #[arg(long = "disable-enclave")]
        disable_enclave: bool,
        #[arg(long = "keystore-path")]
        keystore_path: Option<String>,
        #[arg(long = "enclave-url")]
        enclave_url: Option<String>,
    },
    #[command(about = "Generates BLS key to be used for registering a new validator")]
    Keygen {
        #[arg(long = "module-name")]
        module_name: String,
        #[arg(long = "withdrawal-credentials")]
        withdrawal_credentials: String,
        #[arg(long = "fork-version")]
        fork_version: String,
        #[arg(long = "password-file")]
        password_file: String,
        #[arg(long = "output-file")]
        output_file: String,
    },
    #[cfg(feature = "dev")]
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
    #[cfg(feature = "dev")]
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
    #[cfg(feature = "dev")]
    #[command(about = "Fetch withdrawal credentials for a given module")]
    WithdrawalCredentials {
        #[arg(long = "rpc-url")]
        rpc_url: String,
        #[arg(long = "puffer-protocol-address")]
        puffer_protocol_address: String,
        #[arg(long = "module-address")]
        module_address: String,
    },
    #[command(
        about = "Compute merkle tree rewards from a given rewards file to verify the merkle root posted on chained and the merkle proofs"
    )]
    VerifyMerkleTreeRewards {
        #[arg(long = "rewards-file-path")]
        rewards_file: String,
        #[arg(long = "rpc-url")]
        rpc_url: String,
    },
}

impl ValidatorCommand {
    pub async fn execute(self) -> AppResult<i32> {
        match self {
            Self::GenerateBlsKey {
                password_file,
                output_dir,
            } => {
                generate_bls_key::generate_bls_key(password_file, output_dir).await?;
            }
            Self::ListKeys {
                disable_enclave,
                keystore_path,
                enclave_url,
            } => {
                list_keys::list_keys(disable_enclave, keystore_path, enclave_url).await?;
            }
            Self::Keygen {
                module_name,
                withdrawal_credentials,
                fork_version,
                password_file,
                output_file,
            } => {
                let data = keygen::KeygenCmdInput {
                    module_name,
                    withdrawal_credentials,
                    fork_version,
                    password_file,
                    output_file,
                };
                keygen::keygen_from_cmd(data).await?;
            }
            #[cfg(feature = "dev")]
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
            Self::VerifyMerkleTreeRewards {
                rewards_file,
                rpc_url,
            } => {
                verify_merkle_tree_rewards::verify_merkle_tree_rewards(rewards_file, rpc_url)
                    .await?;
            }
            #[cfg(feature = "dev")]
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
            #[cfg(feature = "dev")]
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
