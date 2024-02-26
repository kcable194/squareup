//! Response struct for the List Payment Links API

use serde::Deserialize;

use super::{errors::Error, PaymentLink};

/// This is a model struct for ListPaymentLinkResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListPaymentLinkResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The list of payment links.
    pub payment_links: Option<Vec<PaymentLink>>,
    /// When a response is truncated, it includes a cursor that you can use in a subsequent request
    /// to retrieve the next set of gift cards. If a cursor is not present, this is the final
    /// response. For more information, see Pagination.
    pub cursor: Option<String>,
}
