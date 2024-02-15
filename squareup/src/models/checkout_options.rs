use serde::{Deserialize, Serialize};

use super::{AcceptedPaymentMethods, CustomField, Money, ShippingFee};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutOptions {
    accepted_payment_methods: Option<AcceptedPaymentMethods>,
    allow_tipping: Option<bool>,
    app_fee_money: Option<Money>,
    ask_for_shipping_address: Option<bool>,
    custom_fields: Option<Vec<CustomField>>,
    merchant_support_email: Option<String>,
    redirect_url: Option<String>,
    shipping_fee: Option<ShippingFee>,
    subscription_plan_id: Option<String>,
}