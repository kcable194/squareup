//! Model struct for CheckoutMerchantSettings type

use crate::models::checkout_merchant_settings_payment_methods::CheckoutMerchantSettingsPaymentMethods;
use crate::models::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutMerchantSettings {
    /// The set of payment methods accepted for the merchant's account.
    pub payment_methods: Option<CheckoutMerchantSettingsPaymentMethods>,
    /// **Read only** The RFC 3339 timestamp specifying the most recent update time of this booking.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time: UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<DateTime>,
}
