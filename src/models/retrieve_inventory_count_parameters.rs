//! Model struct for RetrieveInventoryCountParams type

use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct RetrieveInventoryCountParams {
    /// The Location IDs to look up as a comma-separated list. An empty list queries all locations.
    pub location_ids: Option<Vec<String>>,
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// The page size is currently set to be 100.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
}

impl RetrieveInventoryCountParams {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveInventoryCountParams> for String {
    fn from(retrieve_inventory_count_parameters: RetrieveInventoryCountParams) -> Self {
        retrieve_inventory_count_parameters.to_string()
    }
}

impl Display for RetrieveInventoryCountParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(location_ids) = &self.location_ids {
            params.push(format!("location_ids={}", location_ids.join(",")));
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
