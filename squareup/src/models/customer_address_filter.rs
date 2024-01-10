//! Model struct for CustomerAddressFilter type

use crate::models::enums::Country;
use crate::models::SearchCustomersTextFilter;
use serde::Serialize;

/// The customer address filter.
///
/// This filter is used in a CustomerCustomAttributeFilterValue
/// A type-specific filter used in a custom attribute filter to search based on the value of a
/// customer-related custom attribute.Object filter when searching by an Address-type custom
/// attribute.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CustomerAddressFilter {
    /// The postal code to search for. Only an exact match is supported.
    pub postal_code: Option<SearchCustomersTextFilter>,
    /// The country code to search for.
    pub country: Option<Country>,
}
