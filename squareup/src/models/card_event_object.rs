//! Response body struct for the CardEventObject type

use crate::models::Card;
use serde::{Deserialize, Serialize};

/// This is a model struct for CardEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CardEventObject {
    /// The card associated with the event.
    pub card: Card,
}
