//! Model struct for CustomAttributeDefinition type

use super::DateTime;
use crate::models::enums::CustomAttributeDefinitionVisibility;
use serde::{Deserialize, Serialize};

/// Represents a definition for custom attribute values.
///
/// A custom attribute definition specifies the key, visibility, schema, and other properties for a custom attribute.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CustomAttributeDefinition {
    /// The identifier of the custom attribute definition and its corresponding custom attributes.
    /// This value can be a simple key or a qualified key. The qualified key consists of the application ID
    /// of the custom attribute definition owner followed by the simple key.
    /// Example format: `application_id:simple_key`.
    /// The value for a simple key can contain up to 60 alphanumeric characters, periods (.), underscores (_),
    /// and hyphens (-). This field cannot be changed after creation and must be unique per application, seller,
    /// and resource type.
    pub key: Option<String>,
    /// The JSON schema for the custom attribute definition, which determines the data type of the corresponding
    /// custom attributes. This field is required when creating a definition.
    pub schema: Option<serde_json::Value>,
    /// The name of the custom attribute definition for API and seller-facing UI purposes.
    /// The name must be unique within the seller and application pair. This field is required if the visibility
    /// field is `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.
    pub name: Option<String>,
    /// Seller-oriented description of the custom attribute definition, including any constraints that the seller
    /// should observe. May be displayed as a tooltip in Square UIs.
    /// This field is required if the visibility field is `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.
    pub description: Option<String>,
    /// Specifies how the custom attribute definition and its values should be shared with the seller and other applications.
    /// If no value is specified, the value defaults to `VISIBILITY_HIDDEN`.
    pub visibility: Option<CustomAttributeDefinitionVisibility>,
    /// **Read only** The current version of the custom attribute definition.
    /// This value is incremented each time the custom attribute definition is updated. When updating a custom
    /// attribute definition, you can provide this field and specify the current version of the custom attribute
    /// definition to enable optimistic concurrency.
    /// On writes, this field must be set to the latest version. Stale writes are rejected.
    pub version: Option<i32>,
    /// **Read only** The timestamp that indicates when the custom attribute definition was created
    /// or most recently updated, in RFC 3339 format.
    pub updated_at: Option<DateTime>,
    /// **Read only** The timestamp that indicates when the custom attribute definition was created,
    /// in RFC 3339 format.
    pub created_at: Option<DateTime>,
}
