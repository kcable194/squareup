//! Model struct for SearchCustomersQuery type

use serde::Serialize;

use super::{SearchCustomersFilter, SearchCustomersSort};

/// Contains query criteria for the search.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersQuery {
    /// The filtering criteria for the search query. A query can contain multiple filters in any combination.
    /// Multiple filters are combined as AND statements.
    ///
    /// Note: Combining multiple filters as OR statements is not supported. Instead, send multiple
    /// single-filter searches and join the result sets.
    pub filter: Option<SearchCustomersFilter>,
    /// Sorting criteria for query results. The default behavior is to sort customers alphabetically
    /// by given_name and family_name.
    pub sort: Option<SearchCustomersSort>,
}
