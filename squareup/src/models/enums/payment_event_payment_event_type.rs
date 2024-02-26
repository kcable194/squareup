use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum PaymentEventPaymentEventType {
    #[serde(rename = "payment.created")]
    PaymentCreated,

    #[serde(rename = "payment.updated")]
    PaymentUpdated,
}
