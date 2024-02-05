use std::convert::From;
use std::io;

use ethers::signers::WalletError;

use super::ServerErrorResponse;

#[derive(Clone, Debug)]
pub enum AppErrorKind {
    Io(io::ErrorKind),

    AppError,
    DecodeError,
    ParseError,

    EnvVarError,

    EnclaveError,

    JsonDeError,
    TomlError,

    ContractCallError,

    HyperError,
    ReqwestError,
    ServerError,
    SqlError,

    EthersWalletError,

    UnknownError,
}

impl From<ServerErrorResponse> for AppErrorKind {
    fn from(_err: ServerErrorResponse) -> Self {
        Self::ServerError
    }
}

impl From<io::Error> for AppErrorKind {
    fn from(err: io::Error) -> Self {
        Self::Io(err.kind())
    }
}

impl From<serde_json::Error> for AppErrorKind {
    fn from(_: serde_json::Error) -> Self {
        Self::JsonDeError
    }
}

impl From<std::env::VarError> for AppErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVarError
    }
}

impl From<reqwest::Error> for AppErrorKind {
    fn from(_: reqwest::Error) -> Self {
        Self::ReqwestError
    }
}

impl From<toml::de::Error> for AppErrorKind {
    fn from(_: toml::de::Error) -> Self {
        Self::TomlError
    }
}

impl From<WalletError> for AppErrorKind {
    fn from(_: WalletError) -> Self {
        Self::EthersWalletError
    }
}
