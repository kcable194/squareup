//! Model for CatalogItemProductType enum.

use serde::{Deserialize, Serialize};

/// The type of a CatalogItem.
///
/// Connect V2 only allows the creation of `REGULAR` or `APPOINTMENTS_SERVICE` items.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogItemProductType {
    /// An ordinary item.
    Regular,
    /// A Square gift card.
    #[deprecated]
    GiftCard,
    /// A service that can be booked using the Square Appointments app.
    AppointmentsService,
    /// A food or beverage item that can be sold by restaurants and other food venues.
    FoodAndBev,
    /// An event which tickets can be sold for, including location, address, and times.
    Event,
    /// A digital item like an ebook or song.
    Digital,
    /// A donation which site visitors can send for any cause.
    Donation,
    /// A legacy Square Online service that is manually fulfilled. This corresponds to the Other item type displayed
    /// in the Square Seller Dashboard and Square POS apps.
    LegacySquareOnlineService,
    /// A legacy Square Online membership that is manually fulfilled. This corresponds to the Membership item type
    /// displayed in the Square Seller Dashboard and Square POS apps.
    LegacySquareOnlineMembership,
}
