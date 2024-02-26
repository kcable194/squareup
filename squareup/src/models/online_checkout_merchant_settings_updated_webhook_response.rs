//! Response body struct for the online_checkout.merchant_settings.updated event webhook

use crate::models::enums::OnlineCheckoutMerchantSettingsEventType;
use serde::{Deserialize, Serialize};

use super::{DateTime, MerchantSettingsUpdatedEventData};

/// This is a model struct for OnlineCheckoutMerchantSettingsUpdatedWebhookResponse type.
///
/// Published when online checkout merchant settings are updated
/// Permissions:MERCHANT_PROFILE_WRITE, MERCHANT_PROFILE_READ
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct OnlineCheckoutMerchantSettingsUpdatedWebhookResponse {
    /// The ID of the target merchant associated with the event.
    pub merchant_id: String,
    /// The type of event this represents, "online_checkout.merchant_settings.updated".
    pub r#type: OnlineCheckoutMerchantSettingsEventType,
    /// A unique ID for the event.
    pub event_id: String,
    /// Read only The timestamp of when the event was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: DateTime,
    /// The data associated with the event.
    pub data: MerchantSettingsUpdatedEventData,
}
