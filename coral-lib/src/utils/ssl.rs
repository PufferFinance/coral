use axum::http::StatusCode;
use openssl::x509::X509;

use crate::error::{AppServerResult, ServerErrorResponse};

pub fn extract_x509(signing_cert: &[u8]) -> AppServerResult<(X509, X509)> {
    let x509s = X509::stack_from_pem(signing_cert).map_err(|err| {
        tracing::error!("Failed to parse x509 cert");
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Failed to parse x509 cert",
        )
    })?;

    // Extract intel's signing certificate

    let (signing_x509, root_x509) = match x509s.as_slice() {
        [signing_x509, root_x509] => (signing_x509.to_owned(), root_x509.to_owned()),
        _ => {
            tracing::error!("Failed to split x509 cert");
            return Err(ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                "Failed to split x509 cert",
            ));
        }
    };
    Ok((signing_x509, root_x509))
}

pub fn verify_intel_sgx_attestation_report(x509: &X509) -> AppServerResult<()> {
    match x509
        .subject_name()
        .entries_by_nid(openssl::nid::Nid::COMMONNAME)
        .last()
    {
        Some(name) => {
            let n = name
                .data()
                .as_utf8()
                .map_err(|err| {
                    tracing::debug!("Failed to convert to UTF-8");
                    tracing::debug!("{}", err);
                    ServerErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        1000,
                        "Failed to convert to UTF-8",
                    )
                })?
                .to_string();
            if n.as_str() != "Intel SGX Attestation Report Signing" {
                Err(ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1000,
                    "Invalid attestation",
                ))
            } else {
                Ok(())
            }
        }
        None => Err(ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Invalid attestation",
        )),
    }
}

pub fn verify_intel_sgx_root_ca(x509: &X509) -> AppServerResult<()> {
    match x509
        .subject_name()
        .entries_by_nid(openssl::nid::Nid::COMMONNAME)
        .last()
    {
        Some(name) => {
            let n = name
                .data()
                .as_utf8()
                .map_err(|err| {
                    tracing::debug!("Failed to convert to UTF-8");
                    tracing::debug!("{}", err);
                    ServerErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        1000,
                        "Failed to convert to UTF-8",
                    )
                })?
                .to_string();
            if n.as_str() != "Intel SGX Attestation Report Signing CA" {
                Err(ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1000,
                    "Invalid attestation",
                ))
            } else {
                Ok(())
            }
        }
        None => Err(ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            "Invalid attestation",
        )),
    }
}
