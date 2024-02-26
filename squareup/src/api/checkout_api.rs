//! Accept payments through a pre-built, Square-hosted checkout page. No frontend required.
//!
//! With the Square Checkout API, your customers can pay for a purchase through a simple, Square-hosted
//! checkout page. It can be integrated into any payments workflow with minimal coding.
//!
//! You can create and configure your checkout page through a CreatePaymentLink request, specifying the
//! accepted payment methods and checkout options like tipping and custom fields. You can also configure a
//! URL for customers to be redirected to once they complete their purchase.
//!
//! First time Square developers should utilize the payment link endpoints to create, update, retrieve, and
//! list checkout pages.

use crate::models::{
    ListPaymentLinkResponse, ListPaymentLinksParameters, RetrieveLocationSettingsResponse,
    RetrieveMerchantSettingsResponse, RetrievePaymentLinkResponse, UpdateLocationSettingsRequest,
    UpdateLocationSettingsResponse, UpdateMerchantSettingsRequest, UpdateMerchantSettingsResponse,
    UpdatePaymentLinkRequest, UpdatePaymentLinkResponse,
};
use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::SquareApiError, CreatePaymentLinkRequest, CreatePaymentLinkResponse,
        DeletePaymentLinkResponse,
    },
    SquareClient,
};

const DEFAULT_URI: &str = "/online-checkout";

/// The Checkout API lets developers create and delete Square-hosted checkout links.
pub struct CheckoutApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Payments API endpoints
    client: HttpClient,
}

impl CheckoutApi {
    /// Instantiates a new [`CheckoutApi`]
    pub fn new(square_client: SquareClient) -> Self {
        Self {
            config: square_client.config,
            client: square_client.http_client,
        }
    }

    /// Retrieves the location-level settings for a Square-hosted checkout page.
    /// Permissions:MERCHANT_PROFILE_READ
    pub async fn retrieve_location_settings(
        &self,
        location_id: impl AsRef<str>,
    ) -> Result<RetrieveLocationSettingsResponse, SquareApiError> {
        let url = format!("{}/location-settings/{}", &self.url(), location_id.as_ref());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates the location-level settings for a Square-hosted checkout page.
    /// Permissions:MERCHANT_PROFILE_WRITE, MERCHANT_PROFILE_READ
    pub async fn update_location_settings(
        &self,
        location_id: impl AsRef<str>,
        body: &UpdateLocationSettingsRequest,
    ) -> Result<UpdateLocationSettingsResponse, SquareApiError> {
        let url = format!("{}/location-settings/{}", &self.url(), location_id.as_ref());
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves the merchant-level settings for a Square-hosted checkout page.
    /// Permissions:PAYMENT_METHODS_READ, MERCHANT_PROFILE_READ
    pub async fn retrieve_merchant_settings(
        &self,
    ) -> Result<RetrieveMerchantSettingsResponse, SquareApiError> {
        let url = format!("{}/merchant-settings", &self.url());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates the merchant-level settings for a Square-hosted checkout page.
    /// Permissions:MERCHANT_PROFILE_WRITE, PAYMENT_METHODS_READ, MERCHANT_PROFILE_READ
    pub async fn update_merchant_settings(
        &self,
        body: &UpdateMerchantSettingsRequest,
    ) -> Result<UpdateMerchantSettingsResponse, SquareApiError> {
        let url = format!("{}/merchant-settings", &self.url());
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Lists all payment links.
    /// Permissions:ORDERS_READ
    pub async fn list_payment_links(
        &self,
        params: &ListPaymentLinksParameters,
    ) -> Result<ListPaymentLinkResponse, SquareApiError> {
        let url = format!("{}/payment-links{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a Square-hosted checkout page.
    ///
    /// Applications can share the resulting payment link with their buyer to pay for goods and services.
    pub async fn create_payment_link(
        &self,
        body: &CreatePaymentLinkRequest,
    ) -> Result<CreatePaymentLinkResponse, SquareApiError> {
        let url = format!("{}/payment-links", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes a payment link.
    pub async fn delete_payment_link(
        &self,
        id: impl AsRef<str>,
    ) -> Result<DeletePaymentLinkResponse, SquareApiError> {
        let url = format!("{}/payment-links/{}", &self.url(), id.as_ref());
        let response = self.client.delete(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a payment link.
    /// Permissions:ORDERS_READ
    pub async fn retrieve_payment_link(
        &self,
        id: impl AsRef<str>,
    ) -> Result<RetrievePaymentLinkResponse, SquareApiError> {
        let url = format!("{}/payment-links/{}", &self.url(), id.as_ref());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a payment link.
    ///
    /// You can update the payment_link fields such as description, checkout_options, and
    /// pre_populated_data. You cannot update other fields such as the order_id, version, URL, or
    /// timestamp field.
    /// Permissions:PAYMENTS_WRITE, ORDERS_READ, ORDERS_WRITE
    pub async fn update_payment_link(
        &self,
        id: impl AsRef<str>,
        body: &UpdatePaymentLinkRequest,
    ) -> Result<UpdatePaymentLinkResponse, SquareApiError> {
        let url = format!("{}/payment-links/{}", &self.url(), id.as_ref());
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
