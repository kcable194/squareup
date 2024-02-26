//! Model struct for CheckoutMerchantSettingsPaymentMethods type

use crate::models::{
    CheckoutMerchantSettingsPaymentMethodsAfterpayClearpay,
    CheckoutMerchantSettingsPaymentMethodsPaymentMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutMerchantSettingsPaymentMethods {
    pub apple_pay: Option<CheckoutMerchantSettingsPaymentMethodsPaymentMethod>,
    pub google_pay: Option<CheckoutMerchantSettingsPaymentMethodsPaymentMethod>,
    pub cash_app: Option<CheckoutMerchantSettingsPaymentMethodsPaymentMethod>,
    pub afterpay_clearpay: Option<CheckoutMerchantSettingsPaymentMethodsAfterpayClearpay>,
}
