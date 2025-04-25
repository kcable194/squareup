//! Model struct for RetrieveJobResponse type

use super::{Job, errors::Error};
use serde::Deserialize;

/// This is a model struct for RetrieveJobResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveJobResponse {
    /// The retrieved job.
    pub job: Option<Job>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
