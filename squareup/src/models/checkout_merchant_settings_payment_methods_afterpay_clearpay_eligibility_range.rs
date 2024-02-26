//! Model struct for CheckoutMerchantSettingsPaymentMethodsAfterpayClearpayEligibilityRange type

use crate::models::Money;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutMerchantSettingsPaymentMethodsAfterpayClearpayEligibilityRange {
    pub min: Money,
    pub max: Money,
}
