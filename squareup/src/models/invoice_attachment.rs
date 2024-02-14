//! Model struct for InvoiceAttachment type

use serde::{Deserialize, Serialize};

use super::DateTime;

/// Represents a file attached to an invoice
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoiceAttachment {
    /// **Read only** The Square-assigned ID of the attachment.
    /// per day.
    pub id: Option<String>,
    /// **Read only** The file name of the attachment, which is displayed on the invoice.
    pub filename: Option<String>,
    /// **Read only** The description of the attachment, which is displayed on the invoice. This
    /// field maps to the seller-defined Message field.
    pub description: Option<String>,
    /// **Read only** The file size of the attachment in bytes.
    pub filesize: Option<i32>,
    /// **Read only** The MD5 hash that was generated from the file contents.
    pub hash: Option<String>,
    /// **Read only** The mime type of the attachment. The following mime types are supported:
    /// image/gif, image/jpeg, image/png, image/tiff, image/bmp, application/pdf.
    pub mime_type: Option<String>,
    /// **Read only** The timestamp when the attachment was uploaded, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub uploaded_at: Option<DateTime>,
}
