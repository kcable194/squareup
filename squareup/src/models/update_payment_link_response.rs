//! Response struct for the Update Payment Link API

use serde::Deserialize;

use super::{errors::Error, PaymentLink};

/// This is a model struct for UpdatePaymentLinkResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdatePaymentLinkResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated payment link.
    pub payment_links: Option<Vec<PaymentLink>>,
}
