//! Model struct for ListJobsResponse type

use serde::Deserialize;

use super::{errors::Error, Job};

/// This is a model struct for ListJobsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListJobsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The list of retrieved jobs. A single paged response contains up to 100 jobs.
    pub jobs: Option<Vec<Job>>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the final response.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination)
    pub cursor: Option<String>,
}
