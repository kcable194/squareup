//! Response body struct for the Publish Invoice API.

use serde::Deserialize;

use super::{errors::Error, InvoiceAttachment};

/// This is a model struct for CreateInvoiceAttachmentResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreateInvoiceAttachmentResponse {
    /// Metadata about the attachment that was added to the invoice.
    pub attachment: Option<InvoiceAttachment>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
