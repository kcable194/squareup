//! Response body struct for the InvoiceEventData type

use crate::models::InvoiceEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for InvoiceEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct InvoiceEventData {
    /// The type of the event data object. The value is "invoice". Max Length 50
    pub r#type: String,
    /// The ID of the affected invoice.
    pub id: String,
    /// Indicates that the invoice was deleted. Only applies to "invoice.deleted" event.
    pub deleted: Option<bool>,
    /// An object containing the associated invoice, present if not "invoice.deleted" event.
    pub object: Option<InvoiceEventObject>,
}
