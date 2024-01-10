//! Model struct for PhaseInput type

use serde::{Deserialize, Serialize};

/// Represents the arguments used to construct a new phase.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PhaseInput {
    /// index of phase in total subscription plan
    pub ordinal: i64,
    /// id of order to be used in billing
    pub order_template_id: Option<String>,
}
