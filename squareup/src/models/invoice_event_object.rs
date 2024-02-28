//! Response body struct for the InvoiceEventObject type

use crate::models::Invoice;
use serde::{Deserialize, Serialize};

/// This is a model struct for InvoiceEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct InvoiceEventObject {
    /// The related invoice.
    pub invoice: Invoice,
}
