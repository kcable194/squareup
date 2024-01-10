//! Model struct for CatalogSubscriptionPlanVariation type.

use serde::{Deserialize, Serialize};

use super::SubscriptionPhase;

/// Describes a subscription plan variation.
///
/// A subscription plan variation represents how the subscription for a product or service is
/// sold. For more information, see Subscription Plans and Variations
/// https://developer.squareup.com/docs/subscriptions-api/plans-and-variations
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogSubscriptionPlanVariation {
    /// The name of the plan variation.
    pub name: String,
    /// A list containing each [SubscriptionPhase] for this plan variation.
    pub phases: Option<Vec<SubscriptionPhase>>,
    /// The id of the subscription plan, if there is one.
    pub subscription_plan_id: Option<String>,
    /// The day of the month the billing period starts.
    /// Min 1, Max 31
    pub monthly_billing_anchor_date: Option<i64>,
    /// Whether bills for this plan variation can be split for proration..
    pub can_prorate: Option<bool>,
    /// The ID of a "successor" plan variation to this one. If the field is set, and this object
    /// is disabled at all locations, it indicates that this variation is deprecated and the
    /// object identified by the successor ID be used in its stead.
    pub successor_plan_variation_id: Option<String>,
}
