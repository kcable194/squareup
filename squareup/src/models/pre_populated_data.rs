/// PrePopulatedData : Describes buyer data to prepopulate in the payment form. For more information, see
/// [Optional Checkout Configurations](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations).
use crate::models::Address;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PrePopulatedData {
    /// The buyer email to prepopulate in the payment form.
    pub buyer_email: Option<String>,
    /// The buyer phone number to prepopulate in the payment form.
    pub buyer_phone_number: Option<String>,
    /// The buyer address to prepopulate in the payment form.
    pub buyer_address: Option<Address>,
}

impl Default for PrePopulatedData {
    fn default() -> Self {
        Self::new()
    }
}

impl PrePopulatedData {
    /// Describes buyer data to prepopulate in the payment form. For more information, see [Optional Checkout Configurations](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations).
    pub fn new() -> PrePopulatedData {
        PrePopulatedData {
            buyer_email: None,
            buyer_phone_number: None,
            buyer_address: None,
        }
    }

    pub fn with_buyer_email(mut self, buyer_email: String) -> PrePopulatedData {
        self.buyer_email = Some(buyer_email);
        self
    }

    pub fn reset_buyer_email(&mut self) {
        self.buyer_email = None;
    }

    pub fn with_buyer_phone_number(mut self, buyer_phone_number: String) -> PrePopulatedData {
        self.buyer_phone_number = Some(buyer_phone_number);
        self
    }

    pub fn reset_buyer_phone_number(&mut self) {
        self.buyer_phone_number = None;
    }

    pub fn with_buyer_address(mut self, buyer_address: Address) -> PrePopulatedData {
        self.buyer_address = Some(buyer_address);
        self
    }

    pub fn reset_buyer_address(&mut self) {
        self.buyer_address = None;
    }
}
