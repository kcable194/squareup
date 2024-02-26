//! Response struct for the Retrieve Payment Links API

use serde::Deserialize;

use super::{errors::Error, PaymentLink};

/// This is a model struct for RetrievePaymentLinkResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrievePaymentLinkResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The payment link that is retrieved.
    pub payment_link: Option<PaymentLink>,
}
