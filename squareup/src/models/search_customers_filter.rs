//! Model struct for SearchCustomersFilter type

use serde::Serialize;

use super::{
    CustomerCustomAttributeFilters, FilterValue, SearchCustomerCreationSourceFilter,
    SearchCustomersTextFilter, TimeRange,
};

/// Filtering criteria to use for a `SearchCustomers` request.
///
/// Multiple filters are ANDed together.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersFilter {
    ///The creation source filter.
    /// If one or more creation sources are set, customer profiles are included in,
    ///or excluded from, the result if they match at least one of the filter criteria.
    pub creation_source: Option<SearchCustomerCreationSourceFilter>,
    /// Filter for results within a time range.
    ///
    /// **Important:** If you filter for customers by time range, you must set `SearchCustomersSort` to
    /// sort by the same field. [Learn more about filtering customers by time
    /// range](https://developer.squareup.com/docs/customers-api/manage-customers#important-note-on-filtering-customers-by-time-range).
    pub created_at: Option<TimeRange>,
    /// Similar to created_at, we can also filter with a time range of updated_at property_names
    pub updated_at: Option<TimeRange>,
    /// Filter by [ email addresses]
    pub email_address: Option<SearchCustomersTextFilter>,
    /// Filter by the phone number.
    pub phone_number: Option<SearchCustomersTextFilter>,
    /// Filter by the the secondary id that third party systems can inject when creating the customer.
    pub reference_id: Option<SearchCustomersTextFilter>,
    /// A filter to select customers based on the groups
    /// they belong to. Group membership is controlled by sellers and developers.
    ///
    /// The group_ids filter has the following syntax:
    ///
    /// "group_ids": {
    /// "any":  ["{group_a_id}", "{group_b_id}", ...],
    /// "all":  ["{group_1_id}", "{group_2_id}", ...],
    /// "none": ["{group_i_id}", "{group_ii_id}", ...]
    /// }
    ///
    /// You can use any combination of the any, all, and none fields in the filter. With any, the search
    /// returns customers in groups a or b or any other group specified in the list. With all, the search
    /// returns customers in groups 1 and 2 and all other groups specified in the list. With none, the
    /// search returns customers not in groups i or ii or any other group specified in the list.
    ///
    /// If any of the search conditions are not met, including when an invalid or non-existent group ID is
    /// provided, the result is an empty object ({}).
    pub group_ids: Option<FilterValue>,
    /// A filter to select customers based on one or more custom attributes.
    /// This filter can contain up to 10 custom attribute filters. Each custom attribute filter specifies
    /// filtering criteria for a target custom attribute. If multiple custom attribute filters are provided,
    /// they are combined as an AND operation.
    ///
    /// To be valid for a search, the custom attributes must be visible to the requesting application. For
    /// more information, including example queries, see Search by custom attribute.
    ///
    /// Square returns matching customer profiles, which do not contain custom attributes. To retrieve
    /// customer-related custom attributes, use the Customer Custom Attributes API. For example, you can
    /// call RetrieveCustomerCustomAttribute using a customer ID from the result set.
    pub custom_attribute: Option<CustomerCustomAttributeFilters>,
    /// A filter to select customers based on the segments they belong to. Segment membership is dynamic and
    /// adjusts automatically based on whether customers meet the segment criteria.
    ///
    /// You can provide up to three segment IDs in the filter, using any combination of the all, any, and
    /// none fields. For the following example, the results include customers who belong to both segment A
    /// and segment B but do not belong to segment C.
    ///
    /// "segment_ids": {
    /// "all":  ["{segment_A_id}", "{segment_B_id}"],
    /// "none":  ["{segment_C_id}"]
    /// }
    ///
    /// If an invalid or non-existent segment ID is provided in the filter, Square stops processing the
    /// request and returns a 400 BAD_REQUEST error that includes the segment ID.
    pub segment_ids: Option<FilterValue>,
}
