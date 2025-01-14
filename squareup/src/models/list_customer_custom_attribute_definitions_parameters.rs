//! Model struct for ListCustomerCustomAttributeDefinitionsParameters (query parameters)

use std::fmt::Display;

/// This is a model struct for ListCustomerCustomAttributeDefinitionsParameters (query parameters)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ListCustomerCustomAttributeDefinitionsParameters {
    /// The maximum number of results to return in a single paged response.
    /// Minimum value: 1. Maximum value: 100. Default value: 20.
    pub limit: Option<i32>,
    /// The cursor returned in the paged response from the previous call to this endpoint.
    /// Provide this cursor to retrieve the next page of results for your original request.
    pub cursor: Option<String>,
}

impl ListCustomerCustomAttributeDefinitionsParameters {
    /// Converts the struct to a query string.
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListCustomerCustomAttributeDefinitionsParameters> for String {
    fn from(params: ListCustomerCustomAttributeDefinitionsParameters) -> Self {
        params.to_string()
    }
}

impl Display for ListCustomerCustomAttributeDefinitionsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
