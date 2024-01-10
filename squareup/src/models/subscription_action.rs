//! Model struct for SubscriptionAction type

use crate::models::Phase;
use serde::{Deserialize, Serialize};

use super::enums::SubscriptionActionType;

/// Represents an action as a pending change to a subscription.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriptionAction {
    /// The ID of an action scoped to a subscription.
    pub id: Option<String>,
    /// The type of the action.
    pub r#type: Option<SubscriptionActionType>,
    /// The `YYYY-MM-DD`-formatted date when the action occurs on the subscription.
    pub effective_date: Option<String>,
    /// The new billing anchor day value, for a CHANGE_BILLING_ANCHOR_DATE action.
    pub monthly_billing_anchor_date: Option<i32>,
    /// A list of Phases, to pass phase-specific information used in the swap.
    pub phases: Option<Vec<Phase>>,
    /// The target subscription plan a subscription switches to, for a `SWAP_PLAN` action.
    pub new_plan_variation_id: Option<String>,
}
