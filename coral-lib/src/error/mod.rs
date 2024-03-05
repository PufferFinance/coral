mod error_kind;
mod error_type;
mod server_error;

pub use self::error_kind::AppErrorKind;
pub use self::error_type::AppError;
pub use self::server_error::*;

pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppServerResult<T> = Result<T, ServerErrorResponse>;
