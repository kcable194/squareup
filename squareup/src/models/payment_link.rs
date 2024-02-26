//! Model struct for PaymentRefund type

use crate::models::{CheckoutOptions, DateTime, PrePopulatedData};
use serde::{Deserialize, Serialize};

/// Represents a payment link made using Square.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentLink {
    /// **Read only** The Square-assigned ID of the payment link.
    pub id: Option<String>,
    /// The Square-assigned version number, which is incremented each time an update
    /// is committed to the payment link.
    /// Max 65535
    pub version: i32,
    /// The optional description of the payment_link object.
    /// It is primarily for use by your application and is not used anywhere.
    /// Max Length 4096
    pub description: Option<String>,
    /// **Read only** The ID of the order associated with the payment link.
    /// Max Length 192
    pub order_id: Option<String>,
    /// The checkout options configured for the payment link. For more information, see Optional Checkout Configurations
    pub checkout_options: Option<CheckoutOptions>,
    /// Describes buyer data to prepopulate on the checkout page.
    pub pre_populated_data: Option<PrePopulatedData>,
    /// **Read only** The URL of the payment link.
    pub url: Option<String>,
    /// **Read only** The long URL of the payment link.
    /// Max Length 255
    pub long_url: Option<String>,
    /// The timestamp when the payment link was created, in RFC 3339 format.
    pub created_at: Option<DateTime>,
    /// The timestamp when the payment link was last updated, in RFC 3339 format.
    pub updated_at: Option<DateTime>,
    /// An optional note. After Square processes the payment, this note is added to the resulting Payment.
    /// Max Length 500
    pub payment_note: Option<String>,
}
