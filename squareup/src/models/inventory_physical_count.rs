//! Model struct for InventoryPhysicalCount type

use crate::models::enums::CatalogObjectType;
use serde::{Deserialize, Serialize};

use super::{DateTime, SourceApplication, enums::InventoryState};

/// Represents the quantity of an item variation that is physically present at a specific location,
/// verified by a seller or a seller's employee.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InventoryPhysicalCount {
    /// A unique Square-generated ID for the InventoryPhysicalCount.
    pub id: Option<String>,
    /// An optional ID provided by the application to tie the InventoryPhysicalCount to an external system.
    pub reference_id: Option<String>,
    /// The Square-generated ID of the CatalogObject being tracked.
    pub catalog_object_id: Option<String>,
    /// The type of the CatalogObject being tracked.
    /// The Inventory API supports setting and reading the "catalog_object_type": "ITEM_VARIATION"
    /// In addition, it can also read the "catalog_object_type": "ITEM"
    pub catalog_object_type: Option<CatalogObjectType>,
    /// The current inventory state for the related quantity of items.
    pub state: Option<InventoryState>,
    /// The Square-generated ID of the Location where the related quantity of items is being tracked.
    pub location_id: Option<String>,
    ///The number of items affected by the estimated count as a decimal string.
    pub quantity: Option<String>,
    /// Read only Information about the application with which the physical count is submitted.
    pub source: Option<SourceApplication>,
    /// The Square-generated ID of the Employee responsible for the physical count.
    pub employee_id: Option<String>,
    /// The Square-generated ID of the Team Member responsible for the physical count.
    pub team_member_id: Option<String>,
    /// A client-generated RFC 3339-formatted timestamp that indicates when the
    /// physical count was examined. For physical count updates, the occurred_at timestamp
    /// cannot be older than 24 hours or in the future relative to the time of the request.
    pub occurred_at: Option<DateTime>,
    /// Read only An RFC 3339-formatted timestamp that indicates when the physical count is received.
    pub created_at: Option<DateTime>,
}
