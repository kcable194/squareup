//! Create and manage customer profiles and sync CRM systems with Square.
//!
//! The Customers API enables you to create and manage customer profiles, as well as search for
//! customers based on various criteria (including customer group membership). You can also use the
//! API to sync contacts between your CRM system and Square.

use crate::models::{
    AddGroupToCustomerResponse, BulkCreateCustomersRequest, BulkCreateCustomersResponse,
    BulkDeleteCustomersRequest, BulkDeleteCustomersResponse, BulkRetrieveCustomersRequest,
    BulkRetrieveCustomersResponse, BulkUpdateCustomersRequest, BulkUpdateCustomersResponse,
    DeleteCustomerParameters, RemoveGroupFromCustomerResponse,
};
use crate::{
    SquareClient,
    config::Configuration,
    http::client::HttpClient,
    models::{
        CreateCustomerRequest, CreateCustomerResponse, DeleteCustomerResponse,
        ListCustomersParameters, ListCustomersResponse, RetrieveCustomerResponse,
        SearchCustomersRequest, SearchCustomersResponse, UpdateCustomerRequest,
        UpdateCustomerResponse, errors::SquareApiError,
    },
};

const DEFAULT_URI: &str = "/customers";

/// Create and manage [Customer] profiles and sync CRM systems with Square.
pub struct CustomersApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Customers API endpoints
    http_client: HttpClient,
}

impl CustomersApi {
    /// Instantiates a new `CustomersApi`
    pub fn new(square_client: SquareClient) -> CustomersApi {
        CustomersApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Lists [Customer] profiles associated with a Square account.
    ///
    /// Under normal operating conditions, newly created or updated customer profiles become
    /// available for the listing operation in well under 30 seconds. Occasionally, propagation of
    /// the new or updated profiles can take closer to one minute or longer, especially during
    /// network incidents and outages.
    pub async fn list_customers(
        &self,
        params: &ListCustomersParameters,
    ) -> Result<ListCustomersResponse, SquareApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a new [Customer] for a business.
    ///
    /// You must provide at least one of the following values in your request to this endpoint:
    ///
    /// * `given_name`
    /// * `family_name`
    /// * `company_name`
    /// * `email_address`
    /// * `phone_number`
    pub async fn create_customer(
        &self,
        body: &CreateCustomerRequest,
    ) -> Result<CreateCustomerResponse, SquareApiError> {
        let response = self.http_client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Creates multiple customer profiles for a business.
    ///
    /// This endpoint takes a map of individual create requests and returns a map of responses.
    ///
    /// You must provide at least one of the following values in each create request:
    ///
    /// * `given_name`
    /// * `family_name`
    /// * `company_name`
    /// * `email_address`
    /// * `phone_number`
    ///
    /// Permissions:CUSTOMERS_WRITE
    pub async fn bulk_create_customers(
        &self,
        body: &BulkCreateCustomersRequest,
    ) -> Result<BulkCreateCustomersResponse, SquareApiError> {
        let url = format!("{}/bulk-create", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes multiple customer profiles.
    ///
    /// The endpoint takes a list of customer IDs and returns a map of responses.
    /// Permissions:CUSTOMERS_WRITE
    pub async fn bulk_delete_customers(
        &self,
        body: &BulkDeleteCustomersRequest,
    ) -> Result<BulkDeleteCustomersResponse, SquareApiError> {
        let url = format!("{}/bulk-delete", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves multiple customer profiles.
    ///
    /// This endpoint takes a list of customer IDs and returns a map of responses.
    /// Permissions:CUSTOMERS_READ
    pub async fn bulk_retrieve_customers(
        &self,
        body: &BulkRetrieveCustomersRequest,
    ) -> Result<BulkRetrieveCustomersResponse, SquareApiError> {
        let url = format!("{}/bulk-retrieve", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Updates multiple customer profiles.
    ///
    /// This endpoint takes a map of individual update requests and returns a map of responses.
    ///
    /// You cannot use this endpoint to change cards on file. To make changes, use the Cards API
    /// Use the Cards API to save a credit or debit card on file.Reference or Gift Cards API
    /// Permissions:CUSTOMERS_WRITE
    pub async fn bulk_update_customers(
        &self,
        body: &BulkUpdateCustomersRequest,
    ) -> Result<BulkUpdateCustomersResponse, SquareApiError> {
        let url = format!("{}/bulk-update", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Searches the [Customer] profiles associated with a Square account using a supported query
    /// filter.
    ///
    /// Calling `SearchCustomers` without any explicit query filter returns all customer profiles
    /// ordered alphabetically based on `given_name` and `family_name`.
    ///
    /// Under normal operating conditions, newly created or updated customer profiles become
    /// available for the search operation in well under 30 seconds. Occasionally, propagation of
    /// the new or updated profiles can take closer to one minute or longer, especially during
    /// network incidents and outages.
    pub async fn search_customers(
        &self,
        body: &SearchCustomersRequest,
    ) -> Result<SearchCustomersResponse, SquareApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes a [Customer] profile from a business.
    ///
    /// This operation also unlinks any associated cards on file.
    ///
    /// As a best practice, you should include the version field in the request to enable optimistic
    /// concurrency control. The value must be set to the current version of the customer profile.
    ///
    /// To delete a customer profile that was created by merging existing profiles, you must use the
    /// ID of the newly created profile.
    pub async fn delete_customer(
        &self,
        customer_id: impl AsRef<str>,
        params: &DeleteCustomerParameters,
    ) -> Result<DeleteCustomerResponse, SquareApiError> {
        let url = format!(
            "{}/{}{}",
            &self.url(),
            customer_id.as_ref(),
            params.to_query_string()
        );
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Returns details for a single [Customer].
    pub async fn retrieve_customer(
        &self,
        customer_id: impl AsRef<str>,
    ) -> Result<RetrieveCustomerResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), customer_id.as_ref());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a [Customer] profile.
    ///
    /// To change an attribute, specify the new value. To remove an attribute, specify the value as
    /// an empty string or empty object.
    ///
    /// As a best practice, you should include the `version` field in the request to enable
    /// [optimistic
    /// concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency)
    /// control. The value must be set to the current version of the customer profile.
    ///
    /// To update a customer profile that was created by merging existing profiles, you must use the
    /// ID of the newly created profile.
    ///
    /// You cannot use this endpoint to change cards on file. To make changes, use the [Cards
    /// API](https://developer.squareup.com/reference/square/cards-api) or [Gift Cards
    /// API](https://developer.squareup.com/reference/square/giftcards-api).
    pub async fn update_customer(
        &self,
        customer_id: impl AsRef<str>,
        body: &UpdateCustomerRequest,
    ) -> Result<UpdateCustomerResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), customer_id.as_ref());
        let response = self.http_client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Removes a group membership from a [Customer].
    ///
    /// The customer is identified by the customer_id value and the customer group is
    /// identified by the group_id value.
    pub async fn remove_group_from_customer(
        &self,
        customer_id: impl AsRef<str>,
        group_id: impl AsRef<str>,
    ) -> Result<RemoveGroupFromCustomerResponse, SquareApiError> {
        let url = format!(
            "{}/{}/groups/{}",
            &self.url(),
            customer_id.as_ref(),
            group_id.as_ref()
        );
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Adds a group membership to a [Customer].
    ///
    /// Adds a group membership to a customer.
    /// The customer is identified by the customer_id value and the customer group is identified by
    /// the group_id value.
    pub async fn add_group_to_customer(
        &self,
        customer_id: impl AsRef<str>,
        group_id: impl AsRef<str>,
    ) -> Result<AddGroupToCustomerResponse, SquareApiError> {
        let url = format!(
            "{}/{}/groups/{}",
            &self.url(),
            customer_id.as_ref(),
            group_id.as_ref()
        );
        let response = self.http_client.empty_put(&url).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
