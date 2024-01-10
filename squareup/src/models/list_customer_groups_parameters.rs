//! Query parameters for the List Customer Groups API

use std::fmt::Display;

/// This is a model struct for ListCustomerGroupsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListCustomerGroupsParameters {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for your original query.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// The maximum number of results to return in a single page. This limit is advisory. The
    /// response might contain more or fewer results. If the limit is less than 1 or greater than
    /// 50, Square returns a `400 VALUE_TOO_LOW` or `400 VALUE_TOO_HIGH` error. The default value is
    /// 50.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub limit: Option<i32>,
}

impl ListCustomerGroupsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListCustomerGroupsParameters> for String {
    fn from(list_customer_groups_parameters: ListCustomerGroupsParameters) -> Self {
        list_customer_groups_parameters.to_string()
    }
}

impl Display for ListCustomerGroupsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
