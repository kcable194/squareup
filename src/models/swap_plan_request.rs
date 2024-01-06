//! Request body struct for the Swap Plan API

use serde::Serialize;
use crate::models::PhaseInput;

/// This is a model struct for the SwapPlanRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SwapPlanRequest {
    /// The ID of the new subscription plan variation.
    pub new_plan_variation_id: String,
    /// A list of PhaseInputs, to pass phase-specific information used in the swap.
    pub phases: Option<Vec<PhaseInput>>
}
