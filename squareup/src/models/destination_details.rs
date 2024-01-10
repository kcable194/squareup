//! Model struct for DestinationDetails type

use crate::models::DestinationDetailsCardRefundDetails;
use serde::{Deserialize, Serialize};

/// Details about a refund's destination.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DestinationDetails {
    /// Details about a card refund. Only populated if the destination_type is CARD.
    pub card_details: Option<DestinationDetailsCardRefundDetails>,
}
