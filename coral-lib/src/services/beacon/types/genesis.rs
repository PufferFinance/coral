use ethers::utils::hex::{self};

use serde::Deserialize;

use crate::{
    error::{AppError, AppErrorKind, AppResult},
    structs::eth_types::ForkVersion,
};
use puffersecuresigner::strip_0x_prefix;

#[derive(Clone, Debug, Deserialize)]
pub struct BeaconGenesis {
    pub genesis_time: String,
    pub genesis_validators_root: String,
    pub genesis_fork_version: String,
}

impl BeaconGenesis {
    pub fn fork_version_as_fixed(&self) -> AppResult<ForkVersion> {
        let genesis_fork_version: &str = strip_0x_prefix!(self.genesis_fork_version);
        let genesis_fork_version: ForkVersion = hex::decode(genesis_fork_version)
            .map_err(|_err| {
                AppError::new(
                    AppErrorKind::DecodeError,
                    "Failed to decode hex".to_string(),
                )
            })?
            .as_slice()
            .try_into()
            .map_err(|_err| {
                AppError::new(AppErrorKind::DecodeError, "Invalid length".to_string())
            })?;
        Ok(genesis_fork_version)
    }

    pub fn genesis_validators_root_as_fixed(&self) -> AppResult<[u8; 32]> {
        let genesis_validators_root: &str = strip_0x_prefix!(self.genesis_validators_root);
        let genesis_validators_root: [u8; 32] = hex::decode(genesis_validators_root)
            .map_err(|_err| {
                AppError::new(
                    AppErrorKind::DecodeError,
                    "Failed to decode hex".to_string(),
                )
            })?
            .as_slice()
            .try_into()
            .map_err(|_err| {
                AppError::new(AppErrorKind::DecodeError, "Invalid length".to_string())
            })?;
        Ok(genesis_validators_root)
    }
}
