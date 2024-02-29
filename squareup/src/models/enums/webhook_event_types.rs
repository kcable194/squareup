//! Enum for WebhookEventTypes type.

use crate::models::enums::{
    BookingWebhookEventType, CardWebhookEventType, CatalogWebhookEventType,
    CustomerWebhookEventType, GiftCardActivityWebhookEventType, GiftCardWebhookEventType,
    InventoryWebhookEventType, InvoiceWebhookEventType, LocationWebhookEventType,
    OauthWebhookEventType, OnlineCheckoutLocationSettingsEventType,
    OnlineCheckoutMerchantSettingsEventType, PaymentEventPaymentEventType, RefundWebhookEventType,
    SubscriptionWebhookEventType, TeamMemberWebhookEventType,
};
use crate::models::OrderEventWebhookResponse;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The types of webhook events
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WebhookEventType {
    Booking(BookingWebhookEventType),
    Card(CardWebhookEventType),
    CatalogVersion(CatalogWebhookEventType),
    Customer(CustomerWebhookEventType),
    GiftCardActivity(GiftCardActivityWebhookEventType),
    GiftCard(GiftCardWebhookEventType),
    InventoryCount(InventoryWebhookEventType),
    Invoice(InvoiceWebhookEventType),
    Location(LocationWebhookEventType),
    OauthAuthorization(OauthWebhookEventType),
    OnlineCheckoutLocationSettings(OnlineCheckoutLocationSettingsEventType),
    OnlineCheckoutMerchantSettings(OnlineCheckoutMerchantSettingsEventType),
    Order(OrderEventWebhookResponse),
    Payment(PaymentEventPaymentEventType),
    Refund(RefundWebhookEventType),
    Subscription(SubscriptionWebhookEventType),
    TeamMember(TeamMemberWebhookEventType),
}

impl TryFrom<WebhookEventType> for BookingWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Booking(inner) => Ok(inner),
            _ => Err("WebhookEventType must be variant variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for CardWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Card(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}
impl TryFrom<WebhookEventType> for CatalogWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::CatalogVersion(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for CustomerWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Customer(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}
impl TryFrom<WebhookEventType> for GiftCardActivityWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::GiftCardActivity(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for GiftCardWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::GiftCard(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for InventoryWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::InventoryCount(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for InvoiceWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Invoice(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for LocationWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Location(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for OauthWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::OauthAuthorization(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}
impl TryFrom<WebhookEventType> for OnlineCheckoutLocationSettingsEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::OnlineCheckoutLocationSettings(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for OnlineCheckoutMerchantSettingsEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::OnlineCheckoutMerchantSettings(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}
impl TryFrom<WebhookEventType> for OrderEventWebhookResponse {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Order(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for PaymentEventPaymentEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Payment(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for RefundWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Refund(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for SubscriptionWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::Subscription(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}

impl TryFrom<WebhookEventType> for TeamMemberWebhookEventType {
    type Error = &'static str;

    fn try_from(w: WebhookEventType) -> Result<Self, Self::Error> {
        match w {
            WebhookEventType::TeamMember(inner) => Ok(inner),
            _ => Err("WebhookEventType must be valid variant."),
        }
    }
}
