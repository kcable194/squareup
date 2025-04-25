//! Model struct for CreateJobResponse type

use super::{Job, errors::Error};
use serde::Deserialize;

/// This is a model struct for CreateJobResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CreateJobResponse {
    /// The new job created in the seller account.
    pub job: Option<Job>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
