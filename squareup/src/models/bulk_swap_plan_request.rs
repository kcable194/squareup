//! Request body struct for the Bulk Swap Plan API

use serde::Serialize;

/// This is a model struct for the BulkSwapPlanRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkSwapPlanRequest {
    /// The ID of the new subscription plan.
    pub new_plan_variation_id: String,
    /// The ID of the plan variation whose subscriptions should be swapped. Active subscriptions
    /// using this plan variation will be subscribed to the new plan variation on their next
    /// billing day.
    pub old_plan_variation_id: String,
    /// The ID of the location to associate with the swapped subscriptions.
    pub location_id: String,
}
