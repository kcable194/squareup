//! Response body struct for the order event webhook

use crate::models::enums::{OrderEventObject, OrderWebhookEventType};
use serde::{Deserialize, Serialize};

use super::DateTime;

/// This is a model struct for OrderEventWebhookResponse type.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct OrderEventWebhookResponse {
    /// The ID of the target seller associated with the event.
    pub merchant_id: String,
    /// The type of this event.
    pub r#type: OrderWebhookEventType,
    /// A unique ID for the event.
    pub event_id: String,
    /// Read only The timestamp of when the event was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: DateTime,
    /// The data associated with the event.
    pub data: OrderEventObject,
}

#[cfg(test)]
mod tests {
    use crate::models::enums::OrderEventObject;
    use crate::models::enums::OrderFulfillmentState::{Proposed, Reserved};
    use crate::models::enums::OrderState::Open;
    use crate::models::enums::OrderWebhookEventType;
    use crate::models::order_event_webhook_response::OrderEventWebhookResponse;
    use crate::models::{
        DateTime, OrderCreated, OrderCreatedObject, OrderFulfillmentUpdated,
        OrderFulfillmentUpdatedObject, OrderFulfillmentUpdatedUpdate, OrderUpdated,
        OrderUpdatedObject,
    };

    #[test]
    fn order_created() {
        let order_created_json = r#"
                {
                  "merchant_id": "5S9MXCS9Y99KK",
                  "type": "order.created",
                  "event_id": "116038d3-2948-439f-8679-fc86dbf80f69",
                  "created_at": "2020-04-16T23:14:26.129Z",
                  "data": {
                    "type": "order_created",
                    "id": "eA3vssLHKJrv9H0IdJCM3gNqfdcZY",
                    "object": {
                      "order_created": {
                        "created_at": "2020-04-16T23:14:26.129Z",
                        "location_id": "FPYCBCHYMXFK1",
                        "order_id": "eA3vssLHKJrv9H0IdJCM3gNqfdcZY",
                        "state": "OPEN",
                        "version": 1
                      }
                    }
                  }
                }
            "#;

        let webhook_response = OrderEventWebhookResponse {
            merchant_id: "5S9MXCS9Y99KK".to_string(),
            r#type: OrderWebhookEventType::OrderCreated,
            event_id: "116038d3-2948-439f-8679-fc86dbf80f69".to_string(),
            created_at: DateTime::try_from("2020-04-16T23:14:26.129Z").unwrap(),
            data: OrderEventObject::OrderCreated(OrderCreatedObject {
                order_created: OrderCreated {
                    order_id: "eA3vssLHKJrv9H0IdJCM3gNqfdcZY".to_string(),
                    version: Some(1),
                    location_id: "FPYCBCHYMXFK1".to_string(),
                    state: Open,
                    created_at: DateTime::try_from("2020-04-16T23:14:26.129Z").unwrap(),
                },
            }),
        };

        let deserialized_enum: OrderEventWebhookResponse =
            serde_json::from_str(order_created_json).unwrap();

        assert_eq!(webhook_response, deserialized_enum);
    }

    #[test]
    fn order_fulfillment_updated() {
        let order_fulfillment_updated_json = r#"
                {
                  "merchant_id": "5S9MXCS9Y99KK",
                  "type": "order.fulfillment.updated",
                  "event_id": "b3adf364-4937-436e-a833-49c72b4baee8",
                  "created_at": "2020-04-16T23:16:30.789Z",
                  "data": {
                    "type": "order_fulfillment_updated",
                    "id": "eA3vssLHKJrv9H0IdJCM3gNqfdcZY",
                    "object": {
                      "order_fulfillment_updated": {
                        "created_at": "2020-04-16T23:14:26.129Z",
                        "fulfillment_update": [
                          {
                            "fulfillment_uid": "VWJ1N9leLqjSDLvF2hvYjD",
                            "new_state": "RESERVED",
                            "old_state": "PROPOSED"
                          }
                        ],
                        "location_id": "FPYCBCHYMXFK1",
                        "order_id": "eA3vssLHKJrv9H0IdJCM3gNqfdcZY",
                        "state": "OPEN",
                        "updated_at": "2020-04-16T23:16:30.789Z",
                        "version": 6
                      }
                    }
                  }
                }
            "#;

        let webhook_response = OrderEventWebhookResponse {
            merchant_id: "5S9MXCS9Y99KK".to_string(),
            r#type: OrderWebhookEventType::OrderFulfillmentUpdated,
            event_id: "b3adf364-4937-436e-a833-49c72b4baee8".to_string(),
            created_at: DateTime::try_from("2020-04-16T23:16:30.789Z").unwrap(),
            data: OrderEventObject::OrderFulfillmentUpdated(OrderFulfillmentUpdatedObject {
                order_fulfillment_updated: OrderFulfillmentUpdated {
                    order_id: "eA3vssLHKJrv9H0IdJCM3gNqfdcZY".to_string(),
                    version: Some(6),
                    location_id: "FPYCBCHYMXFK1".to_string(),
                    state: Open,
                    created_at: DateTime::try_from("2020-04-16T23:14:26.129Z").unwrap(),
                    updated_at: DateTime::try_from("2020-04-16T23:16:30.789Z").unwrap(),
                    fulfillment_update: Vec::from([OrderFulfillmentUpdatedUpdate {
                        fulfillment_uid: "VWJ1N9leLqjSDLvF2hvYjD".to_string(),
                        old_state: Some(Proposed),
                        new_state: Reserved,
                    }]),
                },
            }),
        };

        let deserialized_enum: OrderEventWebhookResponse =
            serde_json::from_str(order_fulfillment_updated_json).unwrap();

        assert_eq!(webhook_response, deserialized_enum);
    }

    #[test]
    fn order_updated() {
        let order_updated_json = r#"
                {
                  "merchant_id": "5S9MXCS9Y99KK",
                  "type": "order.updated",
                  "event_id": "4b8e5c91-9f17-4cf1-900a-4a0629f81add",
                  "created_at": "2020-04-16T23:14:26.359Z",
                  "data": {
                    "type": "order_updated",
                    "id": "eA3vssLHKJrv9H0IdJCM3gNqfdcZY",
                    "object": {
                      "order_updated": {
                        "created_at": "2020-04-16T23:14:26.129Z",
                        "location_id": "FPYCBCHYMXFK1",
                        "order_id": "eA3vssLHKJrv9H0IdJCM3gNqfdcZY",
                        "state": "OPEN",
                        "updated_at": "2020-04-16T23:14:26.359Z",
                        "version": 2
                      }
                    }
                  }
                }
            "#;

        let webhook_response = OrderEventWebhookResponse {
            merchant_id: "5S9MXCS9Y99KK".to_string(),
            r#type: OrderWebhookEventType::OrderUpdated,
            event_id: "4b8e5c91-9f17-4cf1-900a-4a0629f81add".to_string(),
            created_at: DateTime::try_from("2020-04-16T23:14:26.359Z").unwrap(),
            data: OrderEventObject::OrderUpdated(OrderUpdatedObject {
                order_updated: OrderUpdated {
                    order_id: "eA3vssLHKJrv9H0IdJCM3gNqfdcZY".to_string(),
                    version: Some(2),
                    location_id: "FPYCBCHYMXFK1".to_string(),
                    state: Open,
                    created_at: DateTime::try_from("2020-04-16T23:14:26.129Z").unwrap(),
                    updated_at: DateTime::try_from("2020-04-16T23:14:26.359Z").unwrap(),
                },
            }),
        };

        let deserialized_enum: OrderEventWebhookResponse =
            serde_json::from_str(order_updated_json).unwrap();

        assert_eq!(webhook_response, deserialized_enum);
    }
}
