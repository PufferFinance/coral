use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u64)]
pub enum ServerErrorCode {
    // config errors
    ConfigError = 100_000,

    //
    EnclaveConnectionError = 400_100,
    EnclaveInvalidRaveEvidence,
    EnclaveInvalidAttestationReport,
    EnclaveInvalidRaveCalldata,

    EnclaveInvalidRegisterKeyData,

    RegisterDataInvalid,

    //
    ParseError = 500_100,

    // http errors
    HttpRequestError,
    HttpParseBodyError,
    HttpUrlError,

    // guardian
    GuardianRotateKeyError,
    GuardianSigningError,
    GuardianInvalidNumberOfSignatures,
    GuardianInvalidSignature,
    GuardianX509Error,
    GuardianKeyNotFound,
    GuardianDuplicateBlsKey,
    GuardianIndexExceedsNextValidatorIndex,

    // guardian db error
    GuardianDbConnectionError,
    GuardianDbInsertError,
    GuardianDbFetchError,
    GuardianDbUpdateError,

    // network error
    EvmFetchChainIdError,
    EvmFetchBlockError,
    EvmFetchLogError,
    EvmFetchTransactionError,
    EvmGetBalanceError,
    EvmSmartContractRevert,
    EvmSendTransactionError,
    EvmWaitForTransactionError,
    EvmMissingTransactionReceipt,

    // db error
    LocalDbConnectionError,
    LocalDbFetchError,
    LocalDbInsertError,
    LocalDbUpdateError,

    // beacon
    BeaconFetchBlockError,
    BeaconFetchBlockRootError,
    BeaconFetchStateRootError,
    BeaconParseBlocKError,

    BeaconFetchValidatorError,
    BeaconParseValidatorErrro,

    BeaconSubmitVoluntaryExitError,

    // puffer errors
    PufferVaultInsufficientETH = 600_100,
}

impl ServerErrorCode {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl Serialize for ServerErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(*self as usize as u64)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSuccessBody<T: Serialize> {
    pub success: bool,
    pub result: T,
}

impl<T: Serialize> ServerSuccessBody<T> {
    pub fn new(result: T) -> Self {
        Self {
            success: true,
            result,
        }
    }
}

impl<T: Serialize> IntoResponse for ServerSuccessBody<T> {
    fn into_response(self) -> Response<body::Body> {
        let json_body = serde_json::to_string(&self).unwrap_or("{}".to_owned());
        let response = Response::builder()
            .header("Content-type", "application/json")
            .status(StatusCode::ACCEPTED)
            .body(body::Body::new(json_body));
        response.unwrap()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerErrorResponse {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub response: ServerErrorBody,
}

impl std::fmt::Display for ServerErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.response.result.message)
    }
}

impl ServerErrorResponse {
    pub fn new(status_code: StatusCode, error_code: ServerErrorCode, message: String) -> Self {
        Self {
            status_code,
            response: ServerErrorBody {
                success: false,
                result: ServerErrorResult {
                    error_code,
                    message,
                },
            },
        }
    }
}

impl IntoResponse for ServerErrorResponse {
    fn into_response(self) -> Response<body::Body> {
        let json_body = serde_json::to_string(&self.response).unwrap_or("{}".to_owned());
        let response = Response::builder()
            .header("Content-type", "application/json")
            .status(self.status_code)
            .body(body::Body::new(json_body));
        response.unwrap()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerErrorBody {
    pub success: bool,
    pub result: ServerErrorResult,
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerErrorResult {
    pub error_code: ServerErrorCode,
    pub message: String,
}
