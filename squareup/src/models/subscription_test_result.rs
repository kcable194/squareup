//! Model struct for SubscriptionTestResult type.

use crate::models::enums::WebhookEventType;
use crate::models::DateTime;
use serde::{Deserialize, Serialize};

/// /// Subscription test result details.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriptionTestResult {
    /// **Read only** A Square-generated unique ID for the subscription test result.
    ///
    /// Max Length: 64
    pub id: Option<String>,
    /// The status code returned by the subscription notification URL.
    pub status_code: Option<i32>,
    /// An object containing the payload of the test event.
    ///
    /// For example, a payment.created event.
    pub payload: Option<WebhookEventType>,
    /// The timestamp of when the subscription was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// - UTC: 2020-01-26T02:25:34Z
    /// - Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<DateTime>,
    /// The timestamp of when the subscription was updated, in RFC 3339 format.
    ///
    /// Because a subscription test result is unique, this field is the same as the created_at field.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// - UTC: 2020-01-26T02:25:34Z
    /// - Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<DateTime>,
}
