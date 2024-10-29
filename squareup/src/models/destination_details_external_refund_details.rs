//! Model struct for DestinationDetailsExternalRefundDetails type

use crate::models::enums::DestinationDetailsExternalRefundDetailsType;
use serde::{Deserialize, Serialize};

/// Stores details about an external refund. Contains only non-confidential information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DestinationDetailsExternalRefundDetails {
    /// The type of external refund the seller paid to the buyer. Max Length 50
    pub r#type: DestinationDetailsExternalRefundDetailsType,
    /// A description of the external refund source. For example, "Food Delivery Service".
    /// Max Length 255
    pub source: String,
    /// An ID to associate the refund to its originating source.
    /// Max Length 255
    pub source_id: Option<String>,
}
