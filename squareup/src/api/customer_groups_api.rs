//! Create and manage customer groups to streamline and automate workflows and help personalize
//! customer interactions.
//!
//! The Customer Groups API lets you create and manage customer groups to provide targeted
//! promotions or take other customized actions based on group membership. For example, you can
//! create Weekly, Monthly, and Quarterly customer groups and add customers to them based on their
//! preferences to receive marketing promotions on a weekly, monthly, and quarterly basis. You can
//! then use the information to manage your marketing email schedule.
//!
//! You can use the Customer Groups API to retrieve and manage customer groups. You can use the
//! Customers API to add customers to and remove customers from groups and search for customers
//! based on group membership.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::SquareApiError, CreateCustomerGroupRequest, CreateCustomerGroupResponse,
        DeleteCustomerGroupResponse, ListCustomerGroupsParameters, ListCustomerGroupsResponse,
        RetrieveCustomerGroupResponse, UpdateCustomerGroupRequest, UpdateCustomerGroupResponse,
    },
    SquareClient,
};

const DEFAULT_URI: &str = "/customers/groups";

/// Create and manage customer groups to streamline and automate workflows and help personalize
/// customer interactions.
pub struct CustomerGroupsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Customer Groups API endpoints
    http_client: HttpClient,
}

impl CustomerGroupsApi {
    /// Instantiates a new `CustomerGroupsApi`
    pub fn new(square_client: SquareClient) -> CustomerGroupsApi {
        CustomerGroupsApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Retrieves the list of customer groups of a business.
    pub async fn list_customer_groups(
        &self,
        params: &ListCustomerGroupsParameters,
    ) -> Result<ListCustomerGroupsResponse, SquareApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a new customer group for a business.
    ///
    /// The request must include the `name` value of the group.
    pub async fn create_customer_group(
        &self,
        body: &CreateCustomerGroupRequest,
    ) -> Result<CreateCustomerGroupResponse, SquareApiError> {
        let response = self.http_client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Deletes a customer group as identified by the `group_id` value.
    pub async fn delete_customer_group(
        &self,
        group_id: impl AsRef<str>,
    ) -> Result<DeleteCustomerGroupResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), group_id.as_ref());
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a specific customer group as identified by the `group_id` value.
    pub async fn retrieve_customer_group(
        &self,
        group_id: impl AsRef<str>,
    ) -> Result<RetrieveCustomerGroupResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), group_id.as_ref());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    pub async fn update_customer_group(
        &self,
        group_id: impl AsRef<str>,
        body: &UpdateCustomerGroupRequest,
    ) -> Result<UpdateCustomerGroupResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), group_id.as_ref());
        let response = self.http_client.put(&url, body).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
