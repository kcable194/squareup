//! Get a list of all a seller's locations.
//!
//! Many sellers use multiple locations to track where they make sales. The Locations API allows you
//! to get data about those locations, such as their addresses, names, and business hours.

use crate::{
    SquareClient,
    config::Configuration,
    http::client::HttpClient,
    models::{
        CreateLocationRequest, CreateLocationResponse, ListLocationsResponse,
        RetrieveLocationResponse, UpdateLocationRequest, UpdateLocationResponse,
        errors::SquareApiError,
    },
};

const DEFAULT_URI: &str = "/locations";

/// Get a list of all a seller's locations.
pub struct LocationsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Locations API endpoints
    http_client: HttpClient,
}

impl LocationsApi {
    /// Instantiates a new `LocationsApi`
    pub fn new(square_client: SquareClient) -> LocationsApi {
        LocationsApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Provides details about all of the seller's
    /// [locations](https://developer.squareup.com/docs/locations-api), including those
    /// with an inactive status.
    pub async fn list_locations(&self) -> Result<ListLocationsResponse, SquareApiError> {
        let response = self.http_client.get(&self.url()).await?;

        response.deserialize().await
    }

    /// Creates a [location](https://developer.squareup.com/docs/locations-api).
    ///
    /// Creating new locations allows for separate configuration of receipt layouts, item prices,
    /// and sales reports. Developers can use locations to separate sales activity through
    /// applications that integrate with Square from sales activity elsewhere in a seller's account.
    /// Locations created programmatically with the Locations API last forever and are visible to
    /// the seller for their own management. Therefore, ensure that each location has a sensible and
    /// unique name.
    pub async fn create_location(
        &self,
        body: &CreateLocationRequest,
    ) -> Result<CreateLocationResponse, SquareApiError> {
        let response = self.http_client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Retrieves details of a single location.
    ///
    /// Specify "main" as the location ID to retrieve details of the [main
    /// location](https://developer.squareup.com/docs/locations-api#about-the-main-location).
    pub async fn retrieve_location(
        &self,
        location_id: impl AsRef<str>,
    ) -> Result<RetrieveLocationResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), location_id.as_ref());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a [location](https://developer.squareup.com/docs/locations-api).
    pub async fn update_location(
        &self,
        location_id: impl AsRef<str>,
        body: &UpdateLocationRequest,
    ) -> Result<UpdateLocationResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), location_id.as_ref());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
