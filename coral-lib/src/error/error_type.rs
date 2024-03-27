use std::convert::From;
use std::io;

#[cfg(feature = "dev")]
use ethers::signers::WalletError;

use super::{AppErrorKind, ServerErrorResponse};

#[derive(Clone, Debug)]
pub struct AppError {
    _kind: AppErrorKind,
    _cause: String,
}

#[allow(dead_code)]
impl AppError {
    pub fn new(_kind: AppErrorKind, _cause: String) -> Self {
        Self { _kind, _cause }
    }

    pub fn kind(&self) -> &AppErrorKind {
        &self._kind
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._cause)
    }
}

impl From<ServerErrorResponse> for AppError {
    fn from(err: ServerErrorResponse) -> Self {
        let cause = err.response.result.message.clone();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}

#[cfg(feature = "dev")]
impl From<WalletError> for AppError {
    fn from(err: WalletError) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}
