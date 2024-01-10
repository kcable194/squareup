//! Model struct for TenderBuyNowPayLaterDetails type

use crate::models::enums::{TenderBuyNowPayLaterDetailsBrand, TenderBuyNowPayLaterDetailsStatus};
use serde::{Deserialize, Serialize};

/// Represents the details of a tender with type BUY_NOW_PAY_LATER.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TenderBuyNowPayLaterDetails {
    /// Read only The Buy Now Pay Later brand.
    pub buy_now_pay_later_brand: Option<TenderBuyNowPayLaterDetailsBrand>,
    /// The buy now pay later payment's current state (such as AUTHORIZED or CAPTURED).
    /// See TenderBuyNowPayLaterDetailsStatus for possible values.
    pub status: Option<TenderBuyNowPayLaterDetailsStatus>,
}
