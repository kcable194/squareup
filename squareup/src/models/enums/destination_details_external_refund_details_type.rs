//! Model for DestinationDetailsExternalRefundDetailsType enum.

use serde::{Deserialize, Serialize};

/// The type of external refund the seller paid to the buyer
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DestinationDetailsExternalRefundDetailsType {
    /// Refunded using a physical check.
    Check,
    /// Refunded using external bank transfer.
    BankTransfer,
    /// Refunded using a non-Square gift card.
    OtherGiftCard,
    /// Refunded using a crypto currency.
    Crypto,
    /// Refunded using Square Cash App.
    SquareCash,
    /// Refunded using peer-to-peer payment applications.
    Social,
    /// A third-party application gathered this refund outside of Square.
    External,
    /// Refunded using an E-money provider.
    Emoney,
    /// A credit or debit card that Square does not support.
    Card,
    /// Use for house accounts, store credit, and so forth.
    StoredBalance,
    /// Restaurant voucher provided by employers to employees to pay for meals
    FoodVoucher,
    /// A type not listed here.
    Other,
}
