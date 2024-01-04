//! Model struct for ListCustomersParameters (query parameters)

use std::fmt::Display;
use super::enums::{SortCustomersField, SortOrder};

const DEFAULT_SORT_ORDER: SortOrder = SortOrder::Asc;
const DEFAULT_SORT_CUSTOMER_FIELD: SortCustomersField = SortCustomersField::CreatedAt;

/// This is a model struct for ListCustomersParameters (query parameters)
#[derive(Clone, Debug)]
pub struct ListCustomersParameters {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this to
    /// retrieve the next set of results for your original query.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: String,
    /// The maximum number of results to return in a single page. This limit is advisory. The response might
    /// contain more or fewer results. If the specified limit is less than 1 or greater than 100, Square returns
    /// a 400 VALUE_TOO_LOW or 400 VALUE_TOO_HIGH error. The default value is 100.
    pub limit: Option<i32>,
    /// Sorts based on a particular field. By default, customers are sorted alphanumerically
    /// by concatenating their given_name and family_name.
    /// If neither name field is set, string comparison is performed using one of the remaining fields
    /// in the following order: company_name, email, phone_number.
    pub sort_field: SortCustomersField,
    /// Sorts the returned list by when the card was created with the specified order. This field
    /// defaults to ASC.
    pub sort_order: SortOrder,
    /// Indicates whether to return the total count of customers in the count field of the response.
    /// The default value is false.
    pub count: Option<bool>,
}

impl ListCustomersParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListCustomersParameters> for String {
    fn from(list_customers_parameters: ListCustomersParameters) -> Self {
        list_customers_parameters.to_string()
    }
}
impl Default for ListCustomersParameters {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            limit: Some(100i32),
            sort_field: DEFAULT_SORT_CUSTOMER_FIELD,
            sort_order: DEFAULT_SORT_ORDER,
            count: Some(false),
        }
    }
}

impl Display for ListCustomersParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if !self.cursor.is_empty() {
            params.push(format!("cursor={}", self.cursor));
        }

        if self.limit.is_some() {
            params.push(format!("limit={}", self.limit.unwrap()));
        }

        if self.sort_field != DEFAULT_SORT_CUSTOMER_FIELD {
            params.push(format!("sort_field={}", self.sort_field));
        }

        if self.sort_order != DEFAULT_SORT_ORDER {
            params.push(format!("sort_order={}", serde_json::to_string(&self.sort_order).unwrap()));
        }

        if self.count.is_some() {
            params.push(format!("count={}", self.count.unwrap()));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
