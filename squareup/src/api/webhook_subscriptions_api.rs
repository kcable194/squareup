//! Create and manage webhook subscriptions.
//!
//! The Webhook Subscriptions API allows you to create, retrieve, update, and delete webhook
//! subscriptions. Because Webhook subscriptions are owned by the application and not any one
//! seller, you cannot use OAuth Access Tokens with the Webhook Subscriptions API. You must use
//! the application’s personal access token
//!
//! For more information, see the following guide the following guide:
//!
//!     Webhook Subscriptions [https://developer.squareup.com/docs/webhooks/webhook-subscriptions-api]

use crate::{
    config::Configuration, http::client::HttpClient, models::errors::SquareApiError, SquareClient,
};
use crate::models::ListWebhookEventTypesResponse;

const DEFAULT_URI: &str = "/webhooks";

pub struct WebhookSubscriptionsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Team API endpoints
    http_client: HttpClient,
}

impl WebhookSubscriptionsApi {
    /// Instantiates a new `WebhookSubscriptionsApi`
    pub fn new(square_client: SquareClient) -> WebhookSubscriptionsApi {
        WebhookSubscriptionsApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Lists all webhook event types that can be subscribed to.
    pub async fn list_webhook_event_types(
        &self,
        api_version: &Option<String>,
    ) -> Result<ListWebhookEventTypesResponse, SquareApiError> {
        let url = match api_version {
            None => { format!("{}/event-types", self.url()) }
            Some(version) => { format!("{}/event-types?api_version={}", self.url(), version) }
        };
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Lists all webhook subscriptions owned by your application.
    pub async fn list_webhook_subscriptions(
        &self,
        params: &ListWebhookSubscriptionsParams,
    ) -> Result<ListWebhookSubscriptionsResponse, SquareApiError> {
        let url = format!("{}/subscriptions{}", self.url(), params.to_query_string());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a webhook subscription.
    pub async fn create_webhook_subscription(
        &self,
        body: &CreateWebhookSubscriptionRequest,
    ) -> Result<CreateWebhookSubscriptionResponse, SquareApiError> {
        let url = format!("{}/subscriptions", self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes a webhook subscription identified by its ID.
    pub async fn delete_webhook_subscription(
        &self,
        subscription_id: impl AsRef<str>,
    ) -> Result<DeleteWebhookSubscriptionResponse, SquareApiError> {
        let url = format!("{}/subscriptions/{}", self.url(), subscription_id.as_ref());
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a webhook subscription identified by its ID.
    pub async fn retrieve_webhook_subscription(
        &self,
        subscription_id: impl AsRef<str>,
    ) -> Result<RetrieveWebhookSubscriptionResponse, SquareApiError> {
        let url = format!("{}/subscriptions/{}", self.url(), subscription_id.as_ref());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a webhook subscription.
    pub async fn update_webhook_subscription(
        &self,
        subscription_id: impl AsRef<str>,
        body: &UpdateWebhookSubscriptionRequest,
    ) -> Result<UpdateWebhookSubscriptionResponse, SquareApiError> {
        let url = format!("{}/subscriptions/{}", self.url(), subscription_id.as_ref());
        let response = self.http_client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Updates a webhook subscription by replacing the existing signature key with a new one.
    pub async fn update_webhook_subscription_signature_key(
        &self,
        subscription_id: impl AsRef<str>,
        body: &UpdateWebhookSubscriptionSignatureKeyRequest,
    ) -> Result<UpdateWebhookSubscriptionSignatureKeyResponse, SquareApiError> {
        let url = format!("{}/subscriptions/{}", self.url(), subscription_id.as_ref());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Tests a webhook subscription by sending a test event to the notification URL.
    pub async fn test_webhook_subscription(
        &self,
        subscription_id: impl AsRef<str>,
        body: &TestWebhookSubscriptionRequest,
    ) -> Result<TestWebhookSubscriptionResponse, SquareApiError> {
        let url = format!("{}/subscriptions/{}/test", self.url(), subscription_id.as_ref());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
