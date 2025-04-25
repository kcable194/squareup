//! Response struct for the Create Payment API

use serde::Deserialize;

use super::Payment;
use super::errors::Error;

/// This is a model struct for CreatePaymentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreatePaymentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created payment.
    pub payment: Payment,
}
