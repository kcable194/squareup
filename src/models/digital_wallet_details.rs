//! Model struct for DigitalWalletDetails type.

use serde::{Deserialize, Serialize};
use crate::models::CashAppDetails;

use super::enums::{CardPaymentDetailsStatus, DigitalWalletBrand};

/// Additional details about `WALLET` type payments.
///
/// Contains only non-confidential information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DigitalWalletDetails {
    /// The status of the `WALLET` payment.
    pub status: Option<CardPaymentDetailsStatus>,
    /// The brand used for the `WALLET` payment.
    pub brand: Option<DigitalWalletBrand>,
    /// Brand-specific details for payments with the brand of CASH_APP.
    pub cash_app_details: Option<CashAppDetails>
}
