use serde::{Deserialize, Serialize};

/// AcceptedPaymentMethods :


#[derive(Copy, Clone,Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AcceptedPaymentMethods {
  /// Whether Apple Pay is accepted at checkout.
  #[serde(rename = "apple_pay")]
  apple_pay: Option<bool>,
  /// Whether Google Pay is accepted at checkout.
  #[serde(rename = "google_pay")]
  google_pay: Option<bool>,
  /// Whether Cash App Pay is accepted at checkout.
  #[serde(rename = "cash_app_pay")]
  cash_app_pay: Option<bool>,
  /// Whether Afterpay/Clearpay is accepted at checkout.
  #[serde(rename = "afterpay_clearpay")]
  afterpay_clearpay: Option<bool>
}

impl AcceptedPaymentMethods {
  /// 
  pub fn new() -> AcceptedPaymentMethods {
    AcceptedPaymentMethods {
      apple_pay: None,
      google_pay: None,
      cash_app_pay: None,
      afterpay_clearpay: None
    }
  }

  pub fn set_apple_pay(&mut self, apple_pay: bool) {
    self.apple_pay = Some(apple_pay);
  }

  pub fn with_apple_pay(mut self, apple_pay: bool) -> AcceptedPaymentMethods {
    self.apple_pay = Some(apple_pay);
    self
  }

  pub fn apple_pay(&self) -> Option<&bool> {
    self.apple_pay.as_ref()
  }

  pub fn reset_apple_pay(&mut self) {
    self.apple_pay = None;
  }

  pub fn set_google_pay(&mut self, google_pay: bool) {
    self.google_pay = Some(google_pay);
  }

  pub fn with_google_pay(mut self, google_pay: bool) -> AcceptedPaymentMethods {
    self.google_pay = Some(google_pay);
    self
  }

  pub fn google_pay(&self) -> Option<&bool> {
    self.google_pay.as_ref()
  }

  pub fn reset_google_pay(&mut self) {
    self.google_pay = None;
  }

  pub fn set_cash_app_pay(&mut self, cash_app_pay: bool) {
    self.cash_app_pay = Some(cash_app_pay);
  }

  pub fn with_cash_app_pay(mut self, cash_app_pay: bool) -> AcceptedPaymentMethods {
    self.cash_app_pay = Some(cash_app_pay);
    self
  }

  pub fn cash_app_pay(&self) -> Option<&bool> {
    self.cash_app_pay.as_ref()
  }

  pub fn reset_cash_app_pay(&mut self) {
    self.cash_app_pay = None;
  }

  pub fn set_afterpay_clearpay(&mut self, afterpay_clearpay: bool) {
    self.afterpay_clearpay = Some(afterpay_clearpay);
  }

  pub fn with_afterpay_clearpay(mut self, afterpay_clearpay: bool) -> AcceptedPaymentMethods {
    self.afterpay_clearpay = Some(afterpay_clearpay);
    self
  }

  pub fn afterpay_clearpay(&self) -> Option<&bool> {
    self.afterpay_clearpay.as_ref()
  }

  pub fn reset_afterpay_clearpay(&mut self) {
    self.afterpay_clearpay = None;
  }

}



