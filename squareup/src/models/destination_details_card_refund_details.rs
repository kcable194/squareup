//! Model struct for DestinationDetailsCardRefundDetails type

use crate::models::enums::CardPaymentDetailsEntryMethod;
use crate::models::Card;
use serde::{Deserialize, Serialize};

/// Details about a refund's card.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DestinationDetailsCardRefundDetails {
    /// The card's non-confidential details.
    pub card: Option<Card>,
    /// The method used to enter the card's details for the refund.
    pub entry_method: Option<CardPaymentDetailsEntryMethod>,
}
