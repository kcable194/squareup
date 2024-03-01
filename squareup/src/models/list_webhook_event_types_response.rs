//! Response struct for the list webhook event types API

use crate::models::enums::WebhookEventType;
use serde::Deserialize;

use super::{errors::Error, EventTypeMetadata};

/// This is a model struct for ListWebhookEventTypesResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListWebhookEventTypesResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The list of event types.
    pub event_types: Option<Vec<WebhookEventType>>,
    /// Contains the metadata of a webhook event type. For more information, see
    /// [EventTypeMetadata][https://developer.squareup.com/reference/square/objects/EventTypeMetadata]
    pub metadata: Option<Vec<EventTypeMetadata>>,
}

#[cfg(test)]
mod tests {
    use crate::models::enums::{
        BookingWebhookEventType, CatalogWebhookEventType, InventoryWebhookEventType,
        InvoiceWebhookEventType, LocationWebhookEventType, PaymentEventPaymentEventType,
        TeamMemberWebhookEventType, WebhookEventType,
    };
    use crate::models::list_webhook_event_types_response::ListWebhookEventTypesResponse;
    use crate::models::EventTypeMetadata;

    #[test]
    fn inventory_count_updated() {
        let response_json = r#"
                {
                  "event_types": [
                    "inventory.count.updated"
                  ],
                  "metadata": [
                    {
                      "event_type": "inventory.count.updated",
                      "api_version_introduced": "2018-07-12",
                      "release_status": "PUBLIC"
                    }
                  ]
                }
            "#;

        let webhook_response = ListWebhookEventTypesResponse {
            errors: None,
            event_types: Some(Vec::from([WebhookEventType::InventoryCount(
                InventoryWebhookEventType::InventoryCountUpdated,
            )])),
            metadata: Some(Vec::from([EventTypeMetadata {
                event_type: Some(WebhookEventType::InventoryCount(
                    InventoryWebhookEventType::InventoryCountUpdated,
                )),
                api_version_introduced: Some("2018-07-12".to_string()),
                release_status: Some("PUBLIC".to_string()),
            }])),
        };

        let deserialized_enum: ListWebhookEventTypesResponse =
            serde_json::from_str(response_json).unwrap();

        let inner_type = deserialized_enum
            .event_types
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
            .try_into()
            .unwrap();

        assert_eq!(webhook_response, deserialized_enum);
        assert_eq!(InventoryWebhookEventType::InventoryCountUpdated, inner_type);
    }

    #[test]
    fn booking_created_event() {
        let response_json = r#"
                {
                  "event_types": [
                    "booking.created"
                  ],
                  "metadata": [
                    {
                      "event_type": "booking.created",
                      "api_version_introduced": "2021-07-12",
                      "release_status": "PUBLIC"
                    }
                  ]
                }
            "#;

        let webhook_response = ListWebhookEventTypesResponse {
            errors: None,
            event_types: Some(Vec::from([WebhookEventType::Booking(
                BookingWebhookEventType::BookingCreated,
            )])),
            metadata: Some(Vec::from([EventTypeMetadata {
                event_type: Some(WebhookEventType::Booking(
                    BookingWebhookEventType::BookingCreated,
                )),
                api_version_introduced: Some("2021-07-12".to_string()),
                release_status: Some("PUBLIC".to_string()),
            }])),
        };

        let deserialized_enum: ListWebhookEventTypesResponse =
            serde_json::from_str(response_json).unwrap();

        let inner_type = deserialized_enum
            .event_types
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
            .try_into()
            .unwrap();

        assert_eq!(webhook_response, deserialized_enum);
        assert_eq!(BookingWebhookEventType::BookingCreated, inner_type);
    }

    #[test]
    fn multiple_events() {
        let response_json = r#"
                {
                  "event_types": [
                    "booking.created",
                    "catalog.version.updated",
                    "location.updated",
                    "payment.created"
                  ],
                  "metadata": [
                    {
                      "event_type": "team_member.updated",
                      "api_version_introduced": "2021-07-12",
                      "release_status": "PUBLIC"
                    },
                    {
                      "event_type": "invoice.scheduled_charge_failed",
                      "api_version_introduced": "2021-07-13",
                      "release_status": "PUBLIC"
                    }
                  ]
                }
            "#;

        let webhook_event_types = Vec::from([
            WebhookEventType::Booking(BookingWebhookEventType::BookingCreated),
            WebhookEventType::CatalogVersion(CatalogWebhookEventType::CatalogVersionUpdated),
            WebhookEventType::Location(LocationWebhookEventType::LocationUpdated),
            WebhookEventType::Payment(PaymentEventPaymentEventType::PaymentCreated),
        ]);

        let webhook_response = ListWebhookEventTypesResponse {
            errors: None,
            event_types: Some(webhook_event_types.clone()),
            metadata: Some(Vec::from([
                EventTypeMetadata {
                    event_type: Some(WebhookEventType::TeamMember(
                        TeamMemberWebhookEventType::TeamMemberUpdated,
                    )),
                    api_version_introduced: Some("2021-07-12".to_string()),
                    release_status: Some("PUBLIC".to_string()),
                },
                EventTypeMetadata {
                    event_type: Some(WebhookEventType::Invoice(
                        InvoiceWebhookEventType::InvoiceScheduledChargeFailed,
                    )),
                    api_version_introduced: Some("2021-07-13".to_string()),
                    release_status: Some("PUBLIC".to_string()),
                },
            ])),
        };

        let deserialized_enum: ListWebhookEventTypesResponse =
            serde_json::from_str(response_json).unwrap();

        assert_eq!(webhook_response, deserialized_enum);

        for (idx, inner) in deserialized_enum
            .event_types
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
        {
            let expected_string = match webhook_event_types.get(idx).unwrap() {
                WebhookEventType::Booking(inner) => inner.to_string(),
                WebhookEventType::Card(inner) => inner.to_string(),
                WebhookEventType::CatalogVersion(inner) => inner.to_string(),
                WebhookEventType::Customer(inner) => inner.to_string(),
                WebhookEventType::GiftCardActivity(inner) => inner.to_string(),
                WebhookEventType::GiftCard(inner) => inner.to_string(),
                WebhookEventType::InventoryCount(inner) => inner.to_string(),
                WebhookEventType::Invoice(inner) => inner.to_string(),
                WebhookEventType::Location(inner) => inner.to_string(),
                WebhookEventType::OauthAuthorization(inner) => inner.to_string(),
                WebhookEventType::OnlineCheckoutLocationSettings(inner) => inner.to_string(),
                WebhookEventType::OnlineCheckoutMerchantSettings(inner) => inner.to_string(),
                WebhookEventType::Order(inner) => inner.to_string(),
                WebhookEventType::Payment(inner) => inner.to_string(),
                WebhookEventType::Refund(inner) => inner.to_string(),
                WebhookEventType::Subscription(inner) => inner.to_string(),
                WebhookEventType::TeamMember(inner) => inner.to_string(),
            };

            let inner_string = match inner {
                WebhookEventType::Booking(inner) => inner.to_string(),
                WebhookEventType::Card(inner) => inner.to_string(),
                WebhookEventType::CatalogVersion(inner) => inner.to_string(),
                WebhookEventType::Customer(inner) => inner.to_string(),
                WebhookEventType::GiftCardActivity(inner) => inner.to_string(),
                WebhookEventType::GiftCard(inner) => inner.to_string(),
                WebhookEventType::InventoryCount(inner) => inner.to_string(),
                WebhookEventType::Invoice(inner) => inner.to_string(),
                WebhookEventType::Location(inner) => inner.to_string(),
                WebhookEventType::OauthAuthorization(inner) => inner.to_string(),
                WebhookEventType::OnlineCheckoutLocationSettings(inner) => inner.to_string(),
                WebhookEventType::OnlineCheckoutMerchantSettings(inner) => inner.to_string(),
                WebhookEventType::Order(inner) => inner.to_string(),
                WebhookEventType::Payment(inner) => inner.to_string(),
                WebhookEventType::Refund(inner) => inner.to_string(),
                WebhookEventType::Subscription(inner) => inner.to_string(),
                WebhookEventType::TeamMember(inner) => inner.to_string(),
            };

            assert_eq!(expected_string, inner_string);
        }
    }
}
