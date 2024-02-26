//! Model struct for CheckoutMerchantSettingsPaymentMethodsPaymentMethod type

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutMerchantSettingsPaymentMethodsPaymentMethod {
    pub enabled: Option<bool>,
}
