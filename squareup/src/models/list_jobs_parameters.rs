//! Model struct for ListJobsParameters (query parameters)

use std::fmt::Display;

/// This is a model struct for ListJobsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListJobsParameters {
    /// The pagination cursor returned by the previous call to this endpoint.
    /// Provide this cursor to retrieve the next page of results for your original request.
    pub cursor: Option<String>,
}

impl ListJobsParameters {
    /// Converts the struct to a query string.
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListJobsParameters> for String {
    fn from(list_jobs_parameters: ListJobsParameters) -> Self {
        list_jobs_parameters.to_string()
    }
}

impl Display for ListJobsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

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
