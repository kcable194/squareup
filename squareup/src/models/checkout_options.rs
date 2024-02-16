use serde::{Deserialize, Serialize};

use super::{AcceptedPaymentMethods, CustomField, Money, ShippingFee};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutOptions {
    /// Indicates whether the payment allows tipping.
    pub allow_tipping: Option<bool>,
    ///The custom fields requesting information from the buyer.
    /// Max Length 2
    pub custom_fields: Option<Vec<CustomField>>,
    /// The ID of the subscription plan for the buyer to pay and subscribe. For more information, see Subscription Plan
    /// Checkout
    /// Max Length 255
    pub subscription_plan_id: Option<String>,
    /// The confirmation page URL to redirect the buyer to after Square processes the payment.
    /// Max Length 2048
    pub redirect_url: Option<String>,
    /// The email address that buyers can use to contact the seller.
    /// Max Length 256
    pub merchant_support_email: Option<String>,
    /// Indicates whether to include the address fields in the payment form.
    pub ask_for_shipping_address: Option<bool>,
    /// The methods allowed for buyers during checkout.
    pub accepted_payment_methods: Option<AcceptedPaymentMethods>,
    /// The amount of money that the developer is taking as a fee for facilitating the payment on behalf of the seller.
    /// The amount cannot be more than 90% of the total amount of the payment.
    ///
    /// The amount must be specified in the smallest denomination of the applicable currency (for example, US dollar
    /// amounts are specified in cents). For more information, see Working with Monetary Amounts
    ///
    /// The fee currency code must match the currency associated with the seller that is accepting the payment. The
    /// application must be from a developer account in the same country and using the same currency code as the seller.
    /// For more information about the application fee scenario, see Take Payments and Collect Fees
    ///
    /// To set this field, PAYMENTS_WRITE_ADDITIONAL_RECIPIENTS OAuth permission is required. For more information,
    /// see Permissions.
    pub app_fee_money: Option<Money>,
    /// The fee associated with shipping to be applied to the Order as a service charge.
    pub shipping_fee: Option<ShippingFee>,
    /// Indicates whether to include the Add coupon section for the buyer to provide a Square marketing coupon in the
    /// payment form.
    pub enable_coupon: Option<bool>,
    /// Indicates whether to include the REWARDS section for the buyer to opt in to loyalty, redeem rewards in the
    /// payment form, or both.
    pub enable_loyalty: Option<bool>,
}
