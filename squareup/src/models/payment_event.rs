use serde::{Deserialize, Serialize};

use super::{enums::PaymentEventPaymentEventType, PaymentEventData};

/// Published when a Payment
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentEvent {
    /// The ID of the target merchant associated with the event.
    merchant_id: Option<String>,

    /// The type of event this represents
    r#type: Option<PaymentEventPaymentEventType>,

    /// A unique ID for the event.
    event_id: Option<String>,

    /// Read only Timestamp of when the event was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    created_at: Option<String>,

    /// Data associated with the event.
    data: Option<PaymentEventData>,
}
