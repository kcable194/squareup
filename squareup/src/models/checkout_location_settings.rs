//! Model struct for CheckoutLocationSettings type

use serde::{Deserialize, Serialize};

use super::{
    CheckoutLocationSettingsBranding, CheckoutLocationSettingsCoupons,
    CheckoutLocationSettingsPolicy, CheckoutLocationSettingsTipping, DateTime,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutLocationSettings {
    /// The ID of the location that these settings apply to.
    pub location_id: Option<String>,
    /// Indicates whether customers are allowed to leave notes at checkout.
    pub customer_notes_enabled: Option<bool>,
    /// Policy information is displayed at the bottom of the checkout pages. You can set a maximum of two policies.
    /// Max Length 2
    pub policies: Option<Vec<CheckoutLocationSettingsPolicy>>,
    /// The branding settings for this location.
    pub branding: Option<CheckoutLocationSettingsBranding>,
    /// The tip settings for this location.
    pub tipping: Option<CheckoutLocationSettingsTipping>,
    /// The coupon settings for this location.
    pub coupons: Option<CheckoutLocationSettingsCoupons>,
    /// **Read only** The RFC 3339 timestamp specifying the most recent update time of this booking.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time: UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<DateTime>,
}
