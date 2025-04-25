//! Response struct for the Create Payment Link API

use serde::Deserialize;

use super::{PaymentLink, PaymentLinkRelatedResources, errors::Error};

/// This is a model struct for CreatePaymentLinkResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreatePaymentLinkResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created payment link.
    pub payment_link: Option<PaymentLink>,
    /// The list of related objects.
    pub related_resources: Option<PaymentLinkRelatedResources>,
}
