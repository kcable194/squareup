//! Subscriptions API
//!
//! Create and manage subscriptions.
//!
//! Subscriptions enable sellers to generate a reliable cash flow and recurring revenue to grow
//! their businesses. Square offers the Subscriptions API for developers to embed subscription
//! functionality in their applications. You first create a subscription plan using the Catalog API
//! and then use the Subscriptions API to create and manage subscriptions.

use crate::models::{
    BulkSwapPlanRequest, BulkSwapPlanResponse, ChangeBillingAnchorDateRequest,
    ChangeBillingAnchorDateResponse,
};
use crate::{
    SquareClient,
    config::Configuration,
    http::client::HttpClient,
    models::{
        CancelSubscriptionResponse, CreateSubscriptionRequest, CreateSubscriptionResponse,
        DeleteSubscriptionActionResponse, ListSubscriptionEventsParameters,
        ListSubscriptionEventsResponse, PauseSubscriptionRequest, PauseSubscriptionResponse,
        ResumeSubscriptionRequest, ResumeSubscriptionResponse, RetrieveSubscriptionParameters,
        RetrieveSubscriptionResponse, SearchSubscriptionsRequest, SearchSubscriptionsResponse,
        SwapPlanRequest, SwapPlanResponse, UpdateSubscriptionRequest, UpdateSubscriptionResponse,
        errors::SquareApiError,
    },
};

const DEFAULT_URI: &str = "/subscriptions";

/// Create and manage subscriptions.
pub struct SubscriptionsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Subscriptions API endpoints
    http_client: HttpClient,
}

impl SubscriptionsApi {
    /// Instantiates a new `SubscriptionsApi`
    pub fn new(square_client: SquareClient) -> SubscriptionsApi {
        SubscriptionsApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Creates a subscription to a subscription plan by a customer.
    ///
    /// If you provide a card on file in the request, Square charges the card for the subscription.
    /// Otherwise, Square bills an invoice to the customer's email address. The subscription starts
    /// immediately, unless the request includes the optional `start_date`. Each individual
    /// subscription is associated with a particular location.
    pub async fn create_subscription(
        &self,
        body: &CreateSubscriptionRequest,
    ) -> Result<CreateSubscriptionResponse, SquareApiError> {
        let response = self.http_client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Schedules a plan variation change for all active subscriptions under a given plan variation.
    pub async fn bulk_swap_plan(
        &self,
        body: &BulkSwapPlanRequest,
    ) -> Result<BulkSwapPlanResponse, SquareApiError> {
        let url = format!("{}/bulk-swap-plan", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Searches for subscriptions.
    ///
    /// Results are ordered chronologically by subscription creation date. If the request specifies
    /// more than one location ID, the endpoint orders the result by location ID, and then by
    /// creation date within each location. If no locations are given in the query, all locations
    /// are searched.
    ///
    /// You can also optionally specify `customer_ids` to search by customer. If left unset, all
    /// customers associated with the specified locations are returned. If the request specifies
    /// customer IDs, the endpoint orders results first by location, within location by customer ID,
    /// and within customer by subscription creation date.
    ///
    /// For more information, see [Retrieve
    /// subscriptions](https://developer.squareup.com/docs/subscriptions-api/overview#retrieve-subscriptions).
    pub async fn search_subscriptions(
        &self,
        body: &SearchSubscriptionsRequest,
    ) -> Result<SearchSubscriptionsResponse, SquareApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a subscription.
    pub async fn retrieve_subscription(
        &self,
        subscription_id: impl AsRef<str>,
        params: &RetrieveSubscriptionParameters,
    ) -> Result<RetrieveSubscriptionResponse, SquareApiError> {
        let url = format!(
            "{}/{}{}",
            &self.url(),
            subscription_id.as_ref(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a subscription.
    ///
    /// You can set, modify, and clear the `subscription` field values.
    pub async fn update_subscription(
        &self,
        subscription_id: impl AsRef<str>,
        body: &UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), subscription_id.as_ref());
        let response = self.http_client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes a scheduled action for a subscription.
    pub async fn delete_subscription_action(
        &self,
        subscription_id: impl AsRef<str>,
        action_id: impl AsRef<str>,
    ) -> Result<DeleteSubscriptionActionResponse, SquareApiError> {
        let url = format!(
            "{}/{}/actions/{}",
            &self.url(),
            subscription_id.as_ref(),
            action_id.as_ref()
        );
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Changes the billing anchor date for a subscription.
    pub async fn change_billing_anchor_date(
        &self,
        subscription_id: impl AsRef<str>,
        body: &ChangeBillingAnchorDateRequest,
    ) -> Result<ChangeBillingAnchorDateResponse, SquareApiError> {
        let url = format!(
            "{}/{}/billing-anchor",
            &self.url(),
            subscription_id.as_ref()
        );
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Schedules a `CANCEL` action to cancel an active subscription by setting the `canceled_date`
    /// field to the end of the active billing period and changing the subscription status from
    /// ACTIVE to CANCELED after this date.
    pub async fn cancel_subscription(
        &self,
        subscription_id: impl AsRef<str>,
    ) -> Result<CancelSubscriptionResponse, SquareApiError> {
        let url = format!("{}/{}/cancel", &self.url(), subscription_id.as_ref());
        let response = self.http_client.empty_post(&url).await?;

        response.deserialize().await
    }

    /// Lists all events for a specific subscription.
    pub async fn list_subscription_events(
        &self,
        subscription_id: impl AsRef<str>,
        params: &ListSubscriptionEventsParameters,
    ) -> Result<ListSubscriptionEventsResponse, SquareApiError> {
        let url = format!(
            "{}/{}/events{}",
            &self.url(),
            subscription_id.as_ref(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Schedules a `PAUSE` action to pause an active subscription.
    pub async fn pause_subscription(
        &self,
        subscription_id: impl AsRef<str>,
        body: &PauseSubscriptionRequest,
    ) -> Result<PauseSubscriptionResponse, SquareApiError> {
        let url = format!("{}/{}/pause", &self.url(), subscription_id.as_ref());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Schedules a `RESUME` action to resume a paused or a deactivated subscription.
    pub async fn resume_subscription(
        &self,
        subscription_id: impl AsRef<str>,
        body: &ResumeSubscriptionRequest,
    ) -> Result<ResumeSubscriptionResponse, SquareApiError> {
        let url = format!("{}/{}/resume", &self.url(), subscription_id.as_ref());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Schedules a `SWAP_PLAN` action to swap a subscription plan in an existing subscription.
    pub async fn swap_plan(
        &self,
        subscription_id: impl AsRef<str>,
        body: &SwapPlanRequest,
    ) -> Result<SwapPlanResponse, SquareApiError> {
        let url = format!("{}/{}/swap-plan", &self.url(), subscription_id.as_ref());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
