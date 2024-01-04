//! Model struct for CatalogSubscriptionPlan type.

use serde::{Deserialize, Serialize};

use super::{CatalogObject, SubscriptionPhase};

/// Describes a subscription plan.
///
/// For more information, see [Set Up and Manage a Subscription
/// Plan](https://developer.squareup.com/docs/subscriptions-api/setup-plan).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogSubscriptionPlan {
    /// The name of the plan.
    pub name: String,
    /// A list of SubscriptionPhase containing the [SubscriptionPhase] for this plan.
    pub phases: Option<Vec<SubscriptionPhase>>,
    /// The list of subscription plan variations available for this product
    pub subscription_plan_variations: Option<Vec<CatalogObject>>,
    /// The list of IDs of CatalogItems that are eligible for subscription by this
    /// SubscriptionPlan's variations.
    pub eligible_item_ids: Option<Vec<String>>,
    /// The list of IDs of CatalogCategory that are eligible for subscription by this
    /// SubscriptionPlan's variations.
    pub eligible_category_ids: Option<Vec<String>>,
    /// If true, all items in the merchant's catalog are subscribable by this SubscriptionPlan.
    pub all_items: Option<bool>,
}
