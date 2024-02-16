use serde::{Deserialize, Serialize};

/// AcceptedPaymentMethods
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AcceptedPaymentMethods {
    /// Whether Apple Pay is accepted at checkout.
    pub apple_pay: Option<bool>,
    /// Whether Google Pay is accepted at checkout.
    pub google_pay: Option<bool>,
    /// Whether Cash App Pay is accepted at checkout.
    pub cash_app_pay: Option<bool>,
    /// Whether Afterpay/Clearpay is accepted at checkout.
    pub afterpay_clearpay: Option<bool>,
}

impl Default for AcceptedPaymentMethods {
    fn default() -> Self {
        Self::new()
    }
}

impl AcceptedPaymentMethods {
    pub fn new() -> AcceptedPaymentMethods {
        AcceptedPaymentMethods {
            apple_pay: None,
            google_pay: None,
            cash_app_pay: None,
            afterpay_clearpay: None,
        }
    }

    pub fn with_apple_pay(mut self, apple_pay: bool) -> AcceptedPaymentMethods {
        self.apple_pay = Some(apple_pay);
        self
    }

    pub fn reset_apple_pay(&mut self) {
        self.apple_pay = None;
    }

    pub fn with_google_pay(mut self, google_pay: bool) -> AcceptedPaymentMethods {
        self.google_pay = Some(google_pay);
        self
    }

    pub fn reset_google_pay(&mut self) {
        self.google_pay = None;
    }

    pub fn with_cash_app_pay(mut self, cash_app_pay: bool) -> AcceptedPaymentMethods {
        self.cash_app_pay = Some(cash_app_pay);
        self
    }

    pub fn reset_cash_app_pay(&mut self) {
        self.cash_app_pay = None;
    }

    pub fn with_afterpay_clearpay(mut self, afterpay_clearpay: bool) -> AcceptedPaymentMethods {
        self.afterpay_clearpay = Some(afterpay_clearpay);
        self
    }

    pub fn reset_afterpay_clearpay(&mut self) {
        self.afterpay_clearpay = None;
    }
}
