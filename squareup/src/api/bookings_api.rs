//! Create and manage bookings for Square sellers.
//!
//! The Bookings API allows you to create, retrieve, update, and cancel appointments online.
//! When used with other Square APIs (such as the Locations API, Team API, Catalog API, and
//! Customers API), the Bookings API lets you create online-booking applications for users to
//! book services provided by Square sellers.

use crate::models::{
    BulkRetrieveBookingsRequest, BulkRetrieveBookingsResponse,
    BulkRetrieveTeamMemberBookingProfilesRequest, BulkRetrieveTeamMemberBookingProfilesResponse,
    CancelBookingRequest, CancelBookingResponse, CreateBookingRequest, CreateBookingResponse,
    ListBookingsParameters, ListBookingsResponse, ListLocationBookingProfilesParameters,
    ListLocationBookingProfilesResponse, ListTeamMemberBookingProfilesParameters,
    ListTeamMemberBookingProfilesResponse, RetrieveBookingResponse,
    RetrieveBusinessBookingProfileResponse, RetrieveLocationBookingProfileResponse,
    RetrieveTeamMemberBookingProfileResponse, SearchAvailabilityRequest,
    SearchAvailabilityResponse, UpdateBookingRequest, UpdateBookingResponse,
};
use crate::{
    config::Configuration, http::client::HttpClient, models::errors::ApiError, SquareClient,
};

const DEFAULT_URI: &str = "/bookings";

/// The Bookings API allows you to create, retrieve, update, and cancel appointments online.
pub struct BookingsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Catalog API endpoints
    http_client: HttpClient,
}

impl BookingsApi {
    /// Instantiates a new `BookingsAPI`
    pub fn new(square_client: SquareClient) -> BookingsApi {
        BookingsApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Retrieve a collection of [Bookings].
    ///
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_READ for
    /// the OAuth scope. To call this endpoint with seller-level permissions, set
    /// APPOINTMENTS_ALL_READ and APPOINTMENTS_READ for the OAuth scope.
    pub async fn list_bookings(
        &self,
        params: &ListBookingsParameters,
    ) -> Result<ListBookingsResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), params.to_query_string());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a booking.
    ///
    /// The required input must include the following:
    ///
    /// Booking.location_id
    /// Booking.start_at
    /// Booking.team_member_id
    /// Booking.AppointmentSegment.service_variation_id
    /// Booking.AppointmentSegment.service_variation_version
    ///
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_WRITE for the OAuth
    /// scope. To call this endpoint with seller-level permissions, set APPOINTMENTS_ALL_WRITE
    /// and APPOINTMENTS_WRITE for the OAuth scope.
    ///
    /// For calls to this endpoint with seller-level permissions to succeed, the seller must have
    /// subscribed to Appointments Plus or Appointments Premium.
    pub async fn create_booking(
        &self,
        body: &CreateBookingRequest,
    ) -> Result<CreateBookingResponse, ApiError> {
        let url = &self.url();
        let response = self.http_client.post(url, body).await?;

        response.deserialize().await
    }

    /// Searches for availabilities for booking.
    ///
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_READ for the OAuth
    /// scope. To call this endpoint with seller-level permissions, set APPOINTMENTS_ALL_READ
    /// and APPOINTMENTS_READ for the OAuth scope.
    /// Permissions: APPOINTMENTS_READ
    pub async fn search_availability(
        &self,
        body: &SearchAvailabilityRequest,
    ) -> Result<SearchAvailabilityResponse, ApiError> {
        let url = format!("{}/availability/search", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Bulk-Retrieves a list of bookings by booking IDs.
    ///
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_READ for the OAuth
    /// scope. To call this endpoint with seller-level permissions, set APPOINTMENTS_ALL_READ and
    /// APPOINTMENTS_READ for the OAuth scope.
    ///
    /// Permissions:APPOINTMENTS_READ
    pub async fn bulk_retrieve_bookings(
        &self,
        body: &BulkRetrieveBookingsRequest,
    ) -> Result<BulkRetrieveBookingsResponse, ApiError> {
        let url = format!("{}/bulk-retrieve", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a seller's booking profile.
    /// Permissions:APPOINTMENTS_BUSINESS_SETTINGS_READ
    pub async fn retrieve_business_booking_profile(
        &self,
    ) -> Result<RetrieveBusinessBookingProfileResponse, ApiError> {
        let url = format!("{}/business-booking-profile", &self.url());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Lists location booking profiles of a seller.
    /// Permissions:APPOINTMENTS_BUSINESS_SETTINGS_READ
    pub async fn list_location_booking_profiles(
        &self,
        params: &ListLocationBookingProfilesParameters,
    ) -> Result<ListLocationBookingProfilesResponse, ApiError> {
        let url = format!(
            "{}/location-booking-profiles{}",
            &self.url(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a seller's location booking profile.
    /// Permissions:APPOINTMENTS_BUSINESS_SETTINGS_READ
    pub async fn retrieve_location_booking_profile(
        &self,
        location_id: &str,
    ) -> Result<RetrieveLocationBookingProfileResponse, ApiError> {
        let url = format!("{}/location-booking-profiles/{location_id}", &self.url());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Lists booking profiles for team members.
    /// Permissions:APPOINTMENTS_BUSINESS_SETTINGS_READ
    pub async fn list_team_member_booking_profiles(
        &self,
        params: &ListTeamMemberBookingProfilesParameters,
    ) -> Result<ListTeamMemberBookingProfilesResponse, ApiError> {
        let url = format!(
            "{}/team-member-booking-profiles{}",
            &self.url(),
            params.to_query_string()
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Retrieves one or more team members' booking profiles.
    /// Permissions:APPOINTMENTS_BUSINESS_SETTINGS_READ
    pub async fn bulk_retrieve_team_member_booking_profiles(
        &self,
        body: &BulkRetrieveTeamMemberBookingProfilesRequest,
    ) -> Result<BulkRetrieveTeamMemberBookingProfilesResponse, ApiError> {
        let url = format!("{}/team-member-booking-profiles/bulk-retrieve", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a team member's booking profile.
    /// Permissions:APPOINTMENTS_BUSINESS_SETTINGS_READ
    pub async fn retrieve_team_member_booking_profile(
        &self,
        team_member_id: &str,
    ) -> Result<RetrieveTeamMemberBookingProfileResponse, ApiError> {
        let url = format!(
            "{}/team-member-booking-profiles/{}",
            &self.url(),
            team_member_id
        );
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a booking.
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_READ for the OAuth
    /// scope. To call this endpoint with seller-level permissions, set APPOINTMENTS_ALL_READ and
    /// APPOINTMENTS_READ for the OAuth scope.
    ///
    /// Permissions:APPOINTMENTS_READ
    pub async fn retrieve_booking(
        &self,
        booking_id: &str,
    ) -> Result<RetrieveBookingResponse, ApiError> {
        let url = format!("{}/{booking_id}", &self.url());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a booking.
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_WRITE for the
    /// OAuth scope. To call this endpoint with seller-level permissions, set
    /// APPOINTMENTS_ALL_WRITE and APPOINTMENTS_WRITE for the OAuth scope.
    ///
    /// For calls to this endpoint with seller-level permissions to succeed, the seller must have
    /// subscribed to Appointments Plus or Appointments Premium.
    ///
    /// Permissions:APPOINTMENTS_WRITE
    pub async fn update_booking(
        &self,
        booking_id: &str,
        body: &UpdateBookingRequest,
    ) -> Result<UpdateBookingResponse, ApiError> {
        let url = format!("{}/{booking_id}", &self.url());
        let response = self.http_client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Cancels an existing booking.
    /// To call this endpoint with buyer-level permissions, set APPOINTMENTS_WRITE for the
    /// OAuth scope. To call this endpoint with seller-level permissions, set
    /// APPOINTMENTS_ALL_WRITE and APPOINTMENTS_WRITE for the OAuth scope.
    ///
    /// For calls to this endpoint with seller-level permissions to succeed, the seller must have
    /// subscribed to Appointments Plus or Appointments Premium.
    ///
    /// Permissions:APPOINTMENTS_WRITE
    pub async fn cancel_booking(
        &self,
        booking_id: &str,
        body: &CancelBookingRequest,
    ) -> Result<CancelBookingResponse, ApiError> {
        let url = format!("{}/{booking_id}/cancel", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
