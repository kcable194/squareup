//! Model struct for CustomAttributeFilter type

use serde::Serialize;
use crate::models::Range;

/// This is a model struct for CustomAttributeFilter type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CustomAttributeFilter {
    /// A query expression to filter items or item variations by matching their custom attributes'
    /// custom_attribute_definition_id property value against the the specified id. Exactly one
    /// of custom_attribute_definition_id or key must be specified.
    pub custom_attribute_definition_id: Option<String>,
    /// A query expression to filter items or item variations by matching their custom attributes'
    /// key property value against the specified key. Exactly one of custom_attribute_definition_id
    /// or key must be specified.
    pub key: Option<String>,
    /// A query expression to filter items or item variations by matching their custom attributes'
    /// string_value property value against the specified text. Exactly one of string_filter,
    /// number_filter, selection_uids_filter, or bool_filter must be specified.
    pub string_filter: Option<String>,
    /// A query expression to filter items or item variations with their custom attributes
    /// containing a number value within the specified range. Exactly one of string_filter,
    /// number_filter, selection_uids_filter, or bool_filter must be specified.
    pub number_filter: Option<Range>,
    /// A query expression to filter items or item variations by matching their custom attributes'
    /// selection_uid_values values against the specified selection uids. Exactly one of
    /// string_filter, number_filter, selection_uids_filter, or bool_filter must be specified.
    pub selection_uids_filter: Option<Vec<String>>,
    /// A query expression to filter items or item variations by matching their custom attributes'
    /// boolean_value property values against the specified Boolean expression. Exactly one of
    /// string_filter, number_filter, selection_uids_filter, or bool_filter must be specified.
    pub bool_filter: Option<bool>,
}