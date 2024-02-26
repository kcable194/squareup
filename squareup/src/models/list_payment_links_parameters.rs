//! Query parameters for the List Payment Links API

use std::fmt::Display;

/// This is a model struct for ListPaymentLinksParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListPaymentLinksParameters {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for the original query. If a cursor is not provided,
    /// the endpoint returns the first page of the results. For more information, see Pagination.
    pub cursor: Option<String>,
    /// A limit on the number of results to return per page. The limit is advisory and the
    /// implementation might return more or less results. If the supplied limit is negative, zero,
    /// or greater than the maximum limit of 1000, it is ignored.
    /// Default value: 100
    pub limit: Option<i32>,
}

impl ListPaymentLinksParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string().replace('\"', "")
    }
}

impl From<ListPaymentLinksParameters> for String {
    fn from(list_payment_links_parameters: ListPaymentLinksParameters) -> Self {
        list_payment_links_parameters.to_string()
    }
}

impl Display for ListPaymentLinksParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = &self.limit {
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
