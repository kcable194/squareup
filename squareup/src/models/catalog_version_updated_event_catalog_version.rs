//! Response body struct for the CatalogVersionUpdatedEventCatalogVersion type

use crate::models::DateTime;
use serde::{Deserialize, Serialize};

/// This is a model struct for CatalogVersionUpdatedEventCatalogVersion type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CatalogVersionUpdatedEventCatalogVersion {
    /// **Read only** Last modification timestamp in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: DateTime,
}
