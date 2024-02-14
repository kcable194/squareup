//! Response body struct for the OauthAuthorizationRevokedEventRevocationObject type

use crate::models::enums::OauthAuthorizationRevokedEventRevokerType;
use crate::models::DateTime;
use serde::{Deserialize, Serialize};

/// This is a model struct for OauthAuthorizationRevokedEventRevocationObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct OauthAuthorizationRevokedEventRevocationObject {
    /// Timestamp of when the revocation event occurred, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub revoked_at: Option<DateTime>,
    /// Type of client that performed the revocation, either APPLICATION, MERCHANT, or SQUARE.
    pub revoker_type: Option<OauthAuthorizationRevokedEventRevokerType>,
}
