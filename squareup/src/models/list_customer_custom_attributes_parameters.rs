//! Model struct for ListCustomerCustomAttributesParameters (query parameters)

use std::fmt::Display;

/// This is a model struct for ListCustomerCustomAttributesParameters (query parameters)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ListCustomerCustomAttributesParameters {
    /// The maximum number of results to return in a single paged response.
    /// Minimum value: 1. Maximum value: 100. Default value: 20.
    pub limit: Option<i32>,
    /// The cursor returned in the paged response from the previous call to this endpoint.
    /// Provide this cursor to retrieve the next page of results for your original request.
    pub cursor: Option<String>,
    /// Indicates whether to return the custom attribute definition in the `definition` field
    /// of each custom attribute. Default: `false`.
    pub with_definitions: Option<bool>,
}

impl ListCustomerCustomAttributesParameters {
    /// Converts the struct to a query string.
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListCustomerCustomAttributesParameters> for String {
    fn from(params: ListCustomerCustomAttributesParameters) -> Self {
        params.to_string()
    }
}

impl Display for ListCustomerCustomAttributesParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(with_definitions) = &self.with_definitions {
            params.push(format!("with_definitions={}", with_definitions));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
