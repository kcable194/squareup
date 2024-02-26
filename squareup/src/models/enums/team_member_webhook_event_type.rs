//! Enum for TeamMemberWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of team member event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TeamMemberWebhookEventType {
    #[serde(rename = "team_member.created")]
    TeamMemberCreated,
    #[serde(rename = "team_member.updated")]
    TeamMemberUpdated,
}

impl Display for TeamMemberWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamMemberWebhookEventType::TeamMemberCreated => {
                write!(f, "team_member.created")
            }
            TeamMemberWebhookEventType::TeamMemberUpdated => {
                write!(f, "team_member.updated")
            }
        }
    }
}
