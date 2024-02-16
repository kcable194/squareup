use crate::models::Money;
use serde::{Deserialize, Serialize};

/// ShippingFee :

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShippingFee {
    /// The name for the shipping fee.
    pub  name: Option<String>,
    /// The amount and currency for the shipping fee.
    pub charge: Money,
}

impl ShippingFee {
    pub fn new(charge: Money) -> ShippingFee {
        ShippingFee { name: None, charge }
    }

    pub fn with_name(mut self, name: String) -> ShippingFee {
        self.name = Some(name);
        self
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn with_charge(mut self, charge: Money) -> ShippingFee {
        self.charge = charge;
        self
    }
}
