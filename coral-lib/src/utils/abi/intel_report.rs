use axum::http::StatusCode;

use ethers::prelude::*;

use puffersecuresigner::io::remote_attestation::AttestationReport;

use crate::{
    error::{AppServerResult, ServerErrorCode, ServerErrorResponse},
    utils,
};

pub fn deserialize_report_from_bytes(b: &[u8]) -> AppServerResult<AttestationReport> {
    let report_string = String::from_utf8(b.to_vec()).map_err(|err| {
        let error_msg = "Failed to parse attestation report to utf8 string";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            err.to_string(),
        )
    })?;
    let report = deserialize_report_from_utf8_string(&report_string)?;
    Ok(report)
}

pub fn deserialize_report_from_utf8_string(b: &str) -> AppServerResult<AttestationReport> {
    let report: AttestationReport = serde_json::from_str(b).map_err(|err| {
        let error_msg = "Failed to parse attestation report";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::EnclaveInvalidAttestationReport,
            err.to_string(),
        )
    })?;
    Ok(report)
}

pub fn to_calldata(attestation_report: &AttestationReport) -> AppServerResult<abi::Bytes> {
    let advisory_ids = serde_json::to_string(&attestation_report.advisoryIDs).map_err(|err| {
        let error_msg = "Failed to serialize advisoryIDs";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            error_msg.to_string(),
        )
    })?;

    let calldata = abi::encode(&[
        abi::Token::Bytes(attestation_report.id.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.timestamp.as_bytes().to_vec()),
        abi::Token::Bytes(
            format!("{}", attestation_report.version)
                .as_bytes()
                .to_vec(),
        ),
        abi::Token::Bytes(attestation_report.epidPseudonym.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.advisoryURL.as_bytes().to_vec()),
        abi::Token::Bytes(advisory_ids.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.isvEnclaveQuoteStatus.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.isvEnclaveQuoteBody.as_bytes().to_vec()),
    ]);
    Ok(calldata)
}

pub fn to_calldata_with_decoded_quote_body(
    attestation_report: &AttestationReport,
) -> AppServerResult<abi::Bytes> {
    let decoded_quote_body =
        utils::encoding::base64_decode_to_bytes(&attestation_report.isvEnclaveQuoteBody)?;

    let advisory_ids = serde_json::to_string(&attestation_report.advisoryIDs).map_err(|err| {
        let error_msg = "Failed to serialize advisoryIDs";
        tracing::error!("{error_msg}: {err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorCode::ParseError,
            error_msg.to_string(),
        )
    })?;

    let calldata = abi::encode(&[
        abi::Token::Bytes(attestation_report.id.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.timestamp.as_bytes().to_vec()),
        abi::Token::Bytes(
            format!("{}", attestation_report.version)
                .as_bytes()
                .to_vec(),
        ),
        abi::Token::Bytes(attestation_report.epidPseudonym.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.advisoryURL.as_bytes().to_vec()),
        abi::Token::Bytes(advisory_ids.as_bytes().to_vec()),
        abi::Token::Bytes(attestation_report.isvEnclaveQuoteStatus.as_bytes().to_vec()),
        abi::Token::Bytes(decoded_quote_body.to_vec()),
    ]);
    Ok(calldata)
}

#[cfg(test)]
mod test {
    use crate::error::AppResult;

    use super::*;
    #[test]
    fn test_decode_intel_report_001() -> AppResult<()> {
        let input = "{\"id\":\"309663105854435152730968447181622581258\",\"timestamp\":\"2024-02-20T19:49:30.408868\",\"version\":4,\"epidPseudonym\":\"EbrM6X6YCH3brjPXT23gVh/I2EG5sVfHYh+S54fb0rrAqVRTiRTOSfLsWSVTZc8wrazGG7oooGoMU7Gj5TEhsmAQ64VNvqTlz0FTUXN4C3Rvk5ZuNLDyRSeHUVisi0qLX3BWJDLOhpHMQ3lPS4LUzoi3Fl+kJmEvTAWyMwCzArY=\",\"advisoryURL\":\"https://security-center.intel.com\",\"advisoryIDs\":[\"INTEL-SA-00334\",\"INTEL-SA-00615\"],\"isvEnclaveQuoteStatus\":\"SW_HARDENING_NEEDED\",\"isvEnclaveQuoteBody\":\"AgABAKwMAAAPAA8AAAAAAEJhbJjVPJcSY5RHybDnAD8AAAAAAAAAAAAAAAAAAAAAFRULB/+ADgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABQAAAAAAAAAfAAAAAAAAAD/errTvf/RxVblofA09Imm7/JcOA2aXkkQCrt+zXW6vAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABenEZ5YPpHGACty67LHfgjAeIZAPvO9OGc4K+1/JQGcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"}";

        let attestation_report = deserialize_report_from_utf8_string(input)?;
        let quote_body = attestation_report.deserialize_quote_body().unwrap();

        assert_eq!(
            "3fdeaeb4ef7ff47155b9687c0d3d2269bbfc970e036697924402aedfb35d6eaf",
            quote_body.MRENCLAVE
        );

        Ok(())
    }
}
