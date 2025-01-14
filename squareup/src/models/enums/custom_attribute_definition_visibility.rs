//! Enum for CustomAttributeDefinitionVisibility type

use serde::{Deserialize, Serialize};

/// Represents the level of permission required to view a custom attribute definition.
///
/// The `visibility` field controls who can read and write the custom attribute values and
/// custom attribute definition.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomAttributeDefinitionVisibility {
    /// The custom attribute definition and values are hidden from the seller (except on export
    /// of all seller data) and other developers.
    VisibilityHidden,
    /// The seller and other developers can read the custom attribute definition and values
    /// on resources.
    VisibilityReadOnly,
    /// The seller and other developers can read the custom attribute definition, and can read
    /// and write values on resources. A custom attribute definition can only be edited or deleted
    /// by the application that created it.
    VisibilityReadWriteValues,
}
