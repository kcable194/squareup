use serde::{Deserialize, Serialize};
use crate::models::Money;

/// ShippingFee :

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShippingFee {
  /// The name for the shipping fee.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The amount and currency for the shipping fee.
  #[serde(rename = "charge")]
  charge: Money
}

impl ShippingFee {
  /// 
  pub fn new(charge: Money) -> ShippingFee {
    ShippingFee {
      name: None,
      charge
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ShippingFee {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_charge(&mut self, charge: Money) {
    self.charge = charge;
  }

  pub fn with_charge(mut self, charge: Money) -> ShippingFee {
    self.charge = charge;
    self
  }

  pub fn charge(&self) -> &Money {
    &self.charge
  }


}



