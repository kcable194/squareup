//! Response struct for the RetrieveInventoryPhysicalCount API

use serde::Deserialize;

use super::{InventoryPhysicalCount, errors::Error};

/// This is a model struct for RetrieveInventoryPhysicalCount type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveInventoryPhysicalCountResponse {
    /// The requested InventoryPhysicalCount.
    pub count: Option<InventoryPhysicalCount>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
