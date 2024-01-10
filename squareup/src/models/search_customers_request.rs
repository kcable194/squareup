//! Model struct for SearchCustomersRequest type

use serde::Serialize;

use super::SearchCustomersQuery;

/// This is a model struct for SearchCustomersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersRequest {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for your original query. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// The maximum number of results to return in a single page. This limit is advisory. The response might
    /// contain more or fewer results. If the specified limit is invalid, Square returns a 400 VALUE_TOO_LOW
    /// or 400 VALUE_TOO_HIGH error. The default value is 100.
    pub limit: Option<i32>,
    /// Query conditions used to filter or sort the results. Note that when retrieving additional
    /// pages using a cursor, you must use the original query.
    pub query: Option<SearchCustomersQuery>,
    /// Indicates whether to return the total count of matching customers in the count field of the response.
    ///
    /// The default value is false.
    pub count: Option<bool>,
}
