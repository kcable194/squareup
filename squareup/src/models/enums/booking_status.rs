//! Model for BookingStatus type.

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// The status of a booking
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingStatus {
    /// An unaccepted booking. It is visible to both sellers and customers.
    Pending,
    /// A customer-cancelled booking. It is visible to both the seller and the customer.
    CancelledByCustomer,
    /// A seller-cancelled booking. It is visible to both the seller and the customer.
    CancelledBySeller,
    /// A declined booking. It had once been pending, but was then declined by the seller.
    Declined,
    /// An accepted booking agreed to or accepted by the seller.
    Accepted,
    /// A no-show booking. The booking was accepted at one time, but have now been marked as
    /// a no-show by the seller because the client either missed the booking or cancelled it
    /// without enough notice.
    NoShow,
}

impl Display for BookingStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingStatus::Pending => { write!(f, "PENDING") }
            BookingStatus::CancelledByCustomer => { write!(f, "CANCELLED_BY_CUSTOMER") }
            BookingStatus::CancelledBySeller => { write!(f, "CANCELLED_BY_SELLER") }
            BookingStatus::Declined => { write!(f, "DECLINED") }
            BookingStatus::Accepted => { write!(f, "ACCEPTED") }
            BookingStatus::NoShow => { write!(f, "NO_SHOW") }
        }
    }
}