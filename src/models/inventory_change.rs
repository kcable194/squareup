//! Model struct for InventoryChange type

use serde::{Deserialize, Serialize};

use super::{
    enums::InventoryChangeType, CatalogMeasurementUnit, InventoryAdjustment,
    InventoryPhysicalCount, InventoryTransfer,
};

/// Changes created for the request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InventoryChange {
    /// Indicates how the inventory change is applied.
    /// See InventoryChangeType for all possible values.
    pub r#type: Option<InventoryChangeType>,
    /// Contains details about the physical count when type is PHYSICAL_COUNT,
    /// and is unset for all other change types.
    pub physical_count: Option<InventoryPhysicalCount>,
    /// Contains details about the inventory adjustment when type is ADJUSTMENT,
    /// and is unset for all other change types.
    pub adjustment: Option<InventoryAdjustment>,
    /// TContains details about the inventory transfer when type is TRANSFER,
    ///  and is unset for all other change types.
    pub transfer: Option<InventoryTransfer>,
    /// Read only The CatalogMeasurementUnit object representing the catalog measurement unit
    pub measurement_unit: Option<CatalogMeasurementUnit>,
    /// ID of the CatalogMeasurementUnit object representing the catalog measurement unit
    pub measurement_unit_id: Option<String>,
}
