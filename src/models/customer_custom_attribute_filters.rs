//! Model struct for CustomerCustomAttributeFilters type

use serde::Serialize;
use crate::models::CustomerCustomAttributeFilter;

/// A filter to select customers based on one or more custom attributes.
/// This filter can contain up to 10 custom attribute filters. Each custom attribute filter
/// specifies filtering criteria for a target custom attribute. If multiple custom attribute
/// filters are provided, they are combined as an AND operation.
///
/// To be valid for a search, the custom attributes must be visible to the requesting
/// application. For more information, including example queries, see Search by custom
/// attribute.
///
/// Square returns matching customer profiles, which do not contain custom attributes.
/// To retrieve customer-related custom attributes, use the Customer Custom Attributes API.
/// For example, you can call RetrieveCustomerCustomAttribute using a customer ID from the
/// result set.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CustomerCustomAttributeFilters {
    /// The custom attribute filters. Each filter must specify key and include the
    /// filter field with a type-specific filter, the updated_at field, or both.
    /// The provided keys must be unique within the list of custom attribute filters.
    pub filters: Option<Vec<CustomerCustomAttributeFilter>>

}