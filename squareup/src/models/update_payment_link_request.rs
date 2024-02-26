//! Request struct for the Update Payment Link API

use serde::Serialize;

use super::PaymentLink;

/// This is a model struct for UpdatePaymentLinkRequest type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UpdatePaymentLinkRequest {
    ///The payment_link object describing the updates to apply. For more information, see Update
    /// a payment link. [https://developer.squareup.com/docs/checkout-api/manage-checkout#update-a-payment-link]
    pub payment_link: PaymentLink,
}
