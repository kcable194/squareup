//! Model struct for OfflinePaymentDetails type

use serde::{Deserialize, Serialize};

/// Details specific to offline payments.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OfflinePaymentDetails {
    /// **Read only** The client-side timestamp of when the offline payment was created, in RFC 3339
    /// format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    /// Max Length 32
    pub client_created_at: Option<String>,
}
