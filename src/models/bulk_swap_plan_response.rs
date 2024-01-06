//! Response body struct for the Bulk Swap Plan API

use serde::Deserialize;

use super::{errors::Error};

/// This is a model struct for the BulkSwapPlanResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkSwapPlanResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The number of affected subscriptions.
    pub affected_subscriptions: Option<i32>,
}
