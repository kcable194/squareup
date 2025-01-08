//! Model struct for UpdateJobRequest type

use super::Job;
use serde::{Serialize};

/// This is a model struct for UpdateJobRequest type
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct UpdateJobRequest {
    /// The job with the updated fields.
    /// Only changed fields need to be included in the request.
    /// Optionally include `version` to enable optimistic concurrency control.
    pub job: Job,
}
