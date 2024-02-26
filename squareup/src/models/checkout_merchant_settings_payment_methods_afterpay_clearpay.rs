//! Model struct for CheckoutMerchantSettingsPaymentMethodsAfterpayClearpay type

use crate::models::CheckoutMerchantSettingsPaymentMethodsAfterpayClearpayEligibilityRange;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutMerchantSettingsPaymentMethodsAfterpayClearpay {
    /// Afterpay is shown as an option for order totals falling within the configured range.
    pub order_eligibility_range:
        Option<CheckoutMerchantSettingsPaymentMethodsAfterpayClearpayEligibilityRange>,
    /// Afterpay is shown as an option for item totals falling within the configured range.
    pub item_eligibility_range:
        Option<CheckoutMerchantSettingsPaymentMethodsAfterpayClearpayEligibilityRange>,
    /// **Read only** Indicates whether the payment method is enabled for the account.
    pub enabled: Option<bool>,
}
