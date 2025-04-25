//! Model for CardIssuerAlert enum

use serde::{Deserialize, Serialize};

/// Indicates the type of issuer alert for a card on file
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardIssuerAlert {
    /// The underlying account of the card was closed, which is a strong signal that future charges
    /// to the card are likely to fail.
    IssuerAlertCardClosed,
}
