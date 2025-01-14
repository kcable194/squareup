//! Use the Customer Custom Attributes API to create and manage custom attributes for customer profiles. Custom attributes can be used to store properties or metadata that simplify integration, synchronization, and personalization workflows. After a custom attribute definition is created in a Square seller account, the custom attribute value can be set for customer profiles in the seller's Customer Directory.
//!
//! For more information, see the following guides:
//!     - Customer Custom Attributes
//!     - Square Webhooks Overview

use crate::models::errors::SquareApiError;
use crate::models::{
    BulkUpsertCustomerCustomAttributesRequest, BulkUpsertCustomerCustomAttributesResponse,
    CreateCustomerCustomAttributeDefinitionRequest,
    CreateCustomerCustomAttributeDefinitionResponse,
    DeleteCustomerCustomAttributeDefinitionResponse, DeleteCustomerCustomAttributeResponse,
    ListCustomerCustomAttributeDefinitionsParameters,
    ListCustomerCustomAttributeDefinitionsResponse, ListCustomerCustomAttributesParameters,
    ListCustomerCustomAttributesResponse, RetrieveCustomerCustomAttributeDefinitionParameters,
    RetrieveCustomerCustomAttributeDefinitionResponse, RetrieveCustomerCustomAttributeParameters,
    RetrieveCustomerCustomAttributeResponse, UpdateCustomerCustomAttributeDefinitionRequest,
    UpdateCustomerCustomAttributeDefinitionResponse, UpsertCustomerCustomAttributeRequest,
    UpsertCustomerCustomAttributeResponse,
};
use crate::{config::Configuration, http::client::HttpClient, SquareClient};

const DEFAULT_URI: &str = "/customers";

/// Create and manage customer-related custom attribute definitions and custom attributes.
pub struct CustomerCustomAttributesApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Customers Custom Attributes API endpoints
    http_client: HttpClient,
}

impl CustomerCustomAttributesApi {
    /// Instantiates a new `CustomerCustomAttributesApi`
    pub fn new(square_client: SquareClient) -> CustomerCustomAttributesApi {
        CustomerCustomAttributesApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Lists the customer-related custom attribute definitions that belong to a Square seller account.
    ///
    /// When all response pages are retrieved, the results include all custom attribute definitions
    /// that are visible to the requesting application, including those created by other applications
    /// and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Seller-defined custom
    /// attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Permissions: CUSTOMERS_READ
    pub async fn list_customer_custom_attribute_definitions(
        &self,
        params: &ListCustomerCustomAttributeDefinitionsParameters,
    ) -> Result<ListCustomerCustomAttributeDefinitionsResponse, SquareApiError> {
        let url = format!(
            "{}/custom-attribute-definitions{}",
            self.url(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a customer-related custom attribute definition for a Square seller account.
    ///
    /// Use this endpoint to define a custom attribute that can be associated with customer profiles.
    /// A custom attribute definition specifies the key, visibility, schema, and other properties for a custom attribute.
    /// After the definition is created, you can call `UpsertCustomerCustomAttribute` or `BulkUpsertCustomerCustomAttributes`
    /// to set the custom attribute for customer profiles in the seller's Customer Directory.
    ///
    /// Permissions: CUSTOMERS_WRITE
    pub async fn create_customer_custom_attribute_definition(
        &self,
        request: &CreateCustomerCustomAttributeDefinitionRequest,
    ) -> Result<CreateCustomerCustomAttributeDefinitionResponse, SquareApiError> {
        let url = format!("{}/custom-attribute-definitions", self.url());
        let response = self.http_client.post(&url, request).await?;

        response.deserialize().await
    }

    /// Deletes a customer-related custom attribute definition from a Square seller account.
    ///
    /// Deleting a custom attribute definition also deletes the corresponding custom attribute from
    /// all customer profiles in the seller's Customer Directory. Only the definition owner can delete
    /// a custom attribute definition.
    ///
    /// Permissions: CUSTOMERS_WRITE
    pub async fn delete_customer_custom_attribute_definition(
        &self,
        key: impl AsRef<str>,
    ) -> Result<DeleteCustomerCustomAttributeDefinitionResponse, SquareApiError> {
        let url = format!(
            "{}/custom-attribute-definitions/{}",
            self.url(),
            key.as_ref()
        );
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a customer-related custom attribute definition from a Square seller account.
    ///
    /// To retrieve a custom attribute definition created by another application, the visibility setting
    /// must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Seller-defined custom attributes
    /// are always set to `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Permissions: CUSTOMERS_READ
    pub async fn retrieve_customer_custom_attribute_definition(
        &self,
        key: impl AsRef<str>,
        params: &RetrieveCustomerCustomAttributeDefinitionParameters,
    ) -> Result<RetrieveCustomerCustomAttributeDefinitionResponse, SquareApiError> {
        let url = format!(
            "{}/custom-attribute-definitions/{}{}",
            self.url(),
            key.as_ref(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a customer-related custom attribute definition for a Square seller account.
    ///
    /// Use this endpoint to update the following fields: `name`, `description`, `visibility`, or the `schema`
    /// for a Selection data type. Only changes to the named options or the maximum number of allowed selections
    /// are supported.
    ///
    /// Only the definition owner can update a custom attribute definition. Sellers can view all custom attributes
    /// in exported customer data, including those set to `VISIBILITY_HIDDEN`.
    ///
    /// Permissions: CUSTOMERS_WRITE
    pub async fn update_customer_custom_attribute_definition(
        &self,
        key: impl AsRef<str>,
        request: &UpdateCustomerCustomAttributeDefinitionRequest,
    ) -> Result<UpdateCustomerCustomAttributeDefinitionResponse, SquareApiError> {
        let url = format!(
            "{}/custom-attribute-definitions/{}",
            self.url(),
            key.as_ref()
        );
        let response = self.http_client.put(&url, request).await?;

        response.deserialize().await
    }

    /// Creates or updates custom attributes for customer profiles as a bulk operation.
    ///
    /// Use this endpoint to set the value of one or more custom attributes for one or more customer profiles.
    /// Permissions: CUSTOMERS_WRITE
    pub async fn bulk_upsert_customer_custom_attributes(
        &self,
        request: &BulkUpsertCustomerCustomAttributesRequest,
    ) -> Result<BulkUpsertCustomerCustomAttributesResponse, SquareApiError> {
        let url = format!("{}//custom-attributes/bulk-upsert", self.url());
        let response = self.http_client.post(&url, request).await?;

        response.deserialize().await
    }

    /// Lists the custom attributes associated with a customer profile.
    ///
    /// Use this endpoint to retrieve all custom attributes visible to the requesting application,
    /// including those owned by other applications and set to `VISIBILITY_READ_ONLY` or
    /// `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Permissions: CUSTOMERS_READ
    pub async fn list_customer_custom_attributes(
        &self,
        customer_id: impl AsRef<str>,
        params: &ListCustomerCustomAttributesParameters,
    ) -> Result<ListCustomerCustomAttributesResponse, SquareApiError> {
        let url = format!(
            "{}/{}/custom-attributes{}",
            self.url(),
            customer_id.as_ref(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Deletes a custom attribute associated with a customer profile.
    ///
    /// To delete a custom attribute owned by another application, the visibility setting must be
    /// `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes are always set to
    /// `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Permissions: CUSTOMERS_WRITE
    pub async fn delete_customer_custom_attribute(
        &self,
        customer_id: impl AsRef<str>,
        key: impl AsRef<str>,
    ) -> Result<DeleteCustomerCustomAttributeResponse, SquareApiError> {
        let url = format!(
            "{}/{}/custom-attributes/{}",
            self.url(),
            customer_id.as_ref(),
            key.as_ref()
        );
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a custom attribute associated with a customer profile.
    ///
    /// Use the `with_definition` query parameter to also retrieve the custom attribute definition
    /// in the same call.
    ///
    /// To retrieve a custom attribute owned by another application, the visibility setting must be
    /// `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Permissions: CUSTOMERS_READ
    pub async fn retrieve_customer_custom_attribute(
        &self,
        customer_id: impl AsRef<str>,
        key: impl AsRef<str>,
        params: &RetrieveCustomerCustomAttributeParameters,
    ) -> Result<RetrieveCustomerCustomAttributeResponse, SquareApiError> {
        let url = format!(
            "{}/{}/custom-attributes/{}{}",
            self.url(),
            customer_id.as_ref(),
            key.as_ref(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates or updates a custom attribute for a customer profile.
    ///
    /// Use this endpoint to set the value of a custom attribute for a specified customer profile.
    /// A custom attribute is based on a custom attribute definition in a Square seller account.
    ///
    /// Permissions: CUSTOMERS_WRITE
    pub async fn upsert_customer_custom_attribute(
        &self,
        customer_id: impl AsRef<str>,
        key: impl AsRef<str>,
        request: &UpsertCustomerCustomAttributeRequest,
    ) -> Result<UpsertCustomerCustomAttributeResponse, SquareApiError> {
        let url = format!(
            "{}/{}/custom-attributes/{}",
            self.url(),
            customer_id.as_ref(),
            key.as_ref()
        );
        let response = self.http_client.post(&url, request).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
