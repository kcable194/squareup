//! Model struct for OrderReturnTip type

use serde::{Deserialize, Serialize};

use super::Money;

/// A tip being returned.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnTip {
    /// A unique ID that identifies the returned discount only within this order.
    /// Max length 60
    pub uid: Option<String>,
    /// **Read only** The amount of tip being returned
    pub applied_money: Option<Money>,
    /// The tender uid from the order that contains the original application of this tip.
    /// Max Length 192
    pub source_tender_uid: Option<String>,
    /// The tender id from the order that contains the original application of this tip.
    /// Max Length 192
    pub source_tender_id: Option<String>,
}
