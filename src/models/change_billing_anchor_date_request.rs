//! Request body struct for the Change billing anchor date API

use serde::Serialize;

/// This is a model struct for the ChangeBillingAnchorDateRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct ChangeBillingAnchorDateRequest {
    /// The anchor day for the billing cycle.
    /// Min 1, Max 31
    pub monthly_billing_anchor_date: Option<i32>,
    /// The YYYY-MM-DD-formatted date when the scheduled BILLING_ANCHOR_CHANGE action takes
    /// place on the subscription.
    ///
    /// When this date is unspecified or falls within the current billing cycle, the billing
    /// anchor date is changed immediately.
    pub effective_date: Option<String>,
}