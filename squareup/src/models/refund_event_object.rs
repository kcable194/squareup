//! Response body struct for the RefundEventObject type

use crate::models::PaymentRefund;
use serde::{Deserialize, Serialize};

/// This is a model struct for RefundEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct RefundEventObject {
    /// The refund associated with the event.
    pub refund: PaymentRefund,
}
