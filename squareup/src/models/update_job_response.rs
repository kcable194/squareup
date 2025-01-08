//! Model struct for UpdateJobResponse type

use super::{errors::Error, Job};
use serde::Deserialize;

/// This is a model struct for UpdateJobResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateJobResponse {
    /// The updated job.
    pub job: Option<Job>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
