use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum PaymentEventPaymentEventType {
    #[serde(rename = "payment.created")]
    PaymentCreated,

    #[serde(rename = "payment.updated")]
    PaymentUpdated,
}

impl Display for PaymentEventPaymentEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentEventPaymentEventType::PaymentCreated => {
                write!(f, "payment.created")
            }
            PaymentEventPaymentEventType::PaymentUpdated => {
                write!(f, "payment.updated")
            }
        }
    }
}
