use serde::{Deserialize, Serialize};

use super::Payment;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentEventObject {
    /// The payment related to this event
    payment: Option<Payment>,
}
