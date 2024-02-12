//! Request body struct for the Publish Invoice API

use serde::Serialize;

/// This is a model struct for CreateInvoiceAttachmentRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateInvoiceAttachmentRequest {
    /// A unique string that identifies the `PublishInvoice` request. If you do not provide
    /// `idempotency_key` (or provide an empty string as the value), the endpoint treats each
    /// request as independent.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency).
    pub idempotency_key: Option<String>,
    /// The description of the attachment to display on the invoice.
    pub description: Option<String>,
}
