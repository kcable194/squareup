//! Model struct for DestinationDetails type

use serde::{Deserialize, Serialize};
use crate::models::DestinationDetailsCardRefundDetails;

/// Details about a refund's destination.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DestinationDetails {
    /// Details about a card refund. Only populated if the destination_type is CARD.
    pub card_details: Option<DestinationDetailsCardRefundDetails>
}