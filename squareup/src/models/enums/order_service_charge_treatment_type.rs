//! Model for OrderServiceChargeTreatmentType enum

use serde::{Deserialize, Serialize};

/// Model for OrderServiceChargeTreatmentType enum
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeTreatmentType {
    LineItemTreatment,
    ApportionedTreatment,
}
