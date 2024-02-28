//! Enum for CatalogWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of catalog event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CatalogWebhookEventType {
    #[serde(rename = "catalog.version.updated")]
    CatalogVersionUpdated,
}

impl Display for CatalogWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogWebhookEventType::CatalogVersionUpdated => {
                write!(f, "catalog.version.updated")
            }
        }
    }
}
