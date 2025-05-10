//! Model struct for CustomAttribute type

use super::{CustomAttributeDefinition, DateTime};
use crate::models::enums::CustomAttributeDefinitionVisibility;
use serde::{Deserialize, Serialize};

/// Represents a custom attribute value.
///
/// Each custom attribute value has a corresponding `CustomAttributeDefinition` object.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CustomAttribute {
    /// The identifier of the custom attribute definition and its corresponding custom attributes.
    /// This value can be a simple key or a qualified key. The qualified key consists of the application ID
    /// of the custom attribute definition owner followed by the simple key.
    /// Example format: `application_id:simple_key`.
    /// The value for a simple key can contain up to 60 alphanumeric characters, periods (.), underscores (_),
    /// and hyphens (-).
    pub key: String,
    /// The value assigned to the custom attribute. It is validated against the custom attribute definition's schema
    /// on write operations.
    pub value: Option<serde_json::Value>,
    /// **Read only**. The current version of the custom attribute. This field is incremented when the custom attribute
    /// is changed. When updating an existing custom attribute value, you can provide this field and specify the current
    /// version of the custom attribute to enable optimistic concurrency.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub version: Option<i32>,
    /// **Read only**. A copy of the visibility field value for the associated custom attribute definition.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub visibility: Option<CustomAttributeDefinitionVisibility>,
    /// **Read only**. A copy of the associated custom attribute definition object. This field is only set
    /// when the optional field is specified on the request.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub definition: Option<CustomAttributeDefinition>,
    /// **Read only**. The timestamp that indicates when the custom attribute was created or most recently updated,
    /// in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub updated_at: Option<DateTime>,
    /// **Read only**. The timestamp that indicates when the custom attribute was created, in RFC 3339 format.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub created_at: Option<DateTime>,
}
