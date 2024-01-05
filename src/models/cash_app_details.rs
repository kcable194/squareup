//! Model struct for CashAppDetails type.

use serde::{Deserialize, Serialize};
use crate::models::enums::Country;

use super::{
    enums::{
        CardPaymentDetailsAvsStatus, CardPaymentDetailsCvvStatus, CardPaymentDetailsEntryMethod,
        CardPaymentDetailsStatus, CardPaymentDetailsVerificationMethod,
        CardPaymentDetailsVerificationResult,
    },
    errors::Error,
    Card, CardPaymentTimeline, DeviceDetails,
};

/// Brand-specific details for payments with the brand of CASH_APP.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CashAppDetails {
    /// The name of the Cash App account holder.
    pub buyer_full_name: Option<String>,
    /// The country of the Cash App account holder, in ISO 3166-1-alpha-2 format.
    /// Min Length 2, Max Length 2
    pub buyer_country_code: Option<Country>,
    /// **Read only** $Cashtag of the Cash App account holder.
    pub buyer_cashtag: Option<String>,
}