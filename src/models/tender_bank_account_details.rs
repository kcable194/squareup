//! Model struct for TenderBankAccountDetails type

use serde::{Deserialize, Serialize};
use crate::models::enums::TenderBankAccountDetailsStatus;

/// Represents the details of a tender with type BANK_ACCOUNT.
///
/// See BankAccountPaymentDetails
/// for more exposed details of a bank account payment.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TenderBankAccountDetails {
    /// The bank account payment's current state.
    pub status: Option<TenderBankAccountDetailsStatus>
}