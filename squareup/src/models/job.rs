//! Model struct for the Job type

use super::DateTime;
use serde::{Deserialize, Serialize};

/// Represents a job that can be assigned to team members.
///
/// This object defines the job's title and tip eligibility. Compensation is defined
/// in a job assignment in a team member's wage setting.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Job {
    /// **Read only** The unique Square-assigned ID of the job.
    /// If you need a job ID for an API request, call ListJobs or use the ID returned when you created the job.
    /// You can also get job IDs from a team member's wage setting.
    pub id: Option<String>,

    /// The title of the job.
    /// Maximum length: 150 characters.
    pub title: Option<String>,

    /// Indicates whether team members can earn tips for the job.
    pub is_tip_eligible: Option<bool>,

    /// **Read only** The timestamp when the job was created, in RFC 3339 format.
    pub created_at: Option<DateTime>,

    /// **Read only** The timestamp when the job was last updated, in RFC 3339 format.
    pub updated_at: Option<DateTime>,

    /// **Read only** The current version of the job.
    /// Include this field in UpdateJob requests to enable optimistic concurrency control
    /// and avoid overwrites from concurrent requests. Requests fail if the provided version
    /// doesn't match the server version at the time of the request.
    pub version: Option<i32>,
}
