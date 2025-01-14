//! Model struct for CreateJobRequest type

use super::Job;
use serde::Serialize;

/// This is a model struct for CreateJobRequest type
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CreateJobRequest {
    /// The job to create. The `title` field is required, and `is_tip_eligible` defaults to true.
    pub job: Job,
    /// A unique identifier for the CreateJob request.
    /// Keys can be any valid string, but must be unique for each request.
    /// For more information, see [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency).
    pub idempotency_key: String,
}
