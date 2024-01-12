//! Error model structs

mod square_api_error;
pub use square_api_error::SquareApiError;

mod error;
pub use error::Error;

mod error_response;
pub use error_response::ErrorResponse;
