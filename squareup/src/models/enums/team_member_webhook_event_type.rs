//! Enum for TeamMemberWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of team member event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TeamMemberWebhookEventType {
    #[serde(rename = "team_member.created")]
    Created,
    #[serde(rename = "team_member.updated")]
    Updated,
}

impl Display for TeamMemberWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamMemberWebhookEventType::Created => {
                write!(f, "team_member.created")
            }
            TeamMemberWebhookEventType::Updated => {
                write!(f, "team_member.updated")
            }
        }
    }
}
