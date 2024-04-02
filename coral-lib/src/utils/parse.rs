use axum::http::StatusCode;

use crate::{
    error::{AppServerResult, ServerErrorCode, ServerErrorResponse},
    strip_0x_prefix,
};

pub fn parse_module_name(module_name: &str) -> AppServerResult<[u8; 32]> {
    let module_name = strip_0x_prefix(module_name);

    let mut module_name_vec: Vec<u8> = hex::decode(module_name).map_err(|err| {
        let error_msg = "Failed to decode module name";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            ServerErrorCode::ParseError,
            err.to_string(),
        )
    })?;

    module_name_vec.resize(32, 0);

    let module_name: [u8; 32] = module_name_vec.as_slice().try_into().map_err(|err| {
        let error_msg = "Module name length incorrect";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            ServerErrorCode::ParseError,
            error_msg.to_string(),
        )
    })?;
    Ok(module_name)
}