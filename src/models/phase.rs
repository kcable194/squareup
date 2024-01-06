//! Model struct for Phase type

use serde::{Deserialize, Serialize};

/// Represents a phase, which can override subscription phases as defined by plan_id
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Phase {
    /// id of subscription phase
    pub uid: Option<String>,
    /// index of phase in total subscription plan
    pub ordinal: Option<i64>,
    /// id of order to be used in billing
    pub order_template_id: Option<String>,
    /// the uid from the plan's phase in catalog
    pub plan_phase_uid: Option<String>,
}