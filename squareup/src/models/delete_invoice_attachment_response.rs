//! Response body struct for the Publish Invoice API.

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteInvoiceAttachmentResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct DeleteInvoiceAttachmentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
