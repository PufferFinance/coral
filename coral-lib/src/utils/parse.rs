use axum::http::StatusCode;

use ethers::utils::hex::{self};

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    strip_0x_prefix,
};

pub fn parse_module_name(module_name: &str) -> AppServerResult<[u8; 32]> {
    let mut module_name: String = strip_0x_prefix!(&module_name);
    if module_name.len() > 64 {
        let err = ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            &format!("Invalid module value: '{}'", module_name),
        );
        return Err(err);
    }

    for _ in module_name.len()..64 {
        module_name.push('0');
    }

    let module_name_vec: Vec<u8> = hex::decode(&module_name).map_err(|err| {
        let error_msg = "Failed to decode module name";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;
    let module_name: [u8; 32] = module_name_vec.as_slice().try_into().map_err(|err| {
        let error_msg = "Module name length incorrect";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })?;
    Ok(module_name)
}

pub fn parse_withdrawal_credentials(val: &[u8]) -> AppServerResult<[u8; 32]> {
    let withdrawal_credentials: [u8; 32] = val.try_into().map_err(|err| {
        let error_msg = "Failed to parse withdrawal_credentials";
        tracing::error!("{error_msg}");
        tracing::error!("{err}");
        ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1000, error_msg)
    })?;
    Ok(withdrawal_credentials)
}
