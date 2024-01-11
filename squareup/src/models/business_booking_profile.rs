//! Model struct for BusinessBookingProfile type

use crate::models::enums::{
    BusinessBookingProfileBookingPolicy, BusinessBookingProfileCustomerTimezoneChoice,
};
use crate::models::{BusinessAppointmentSettings, DateTime};
use serde::{Deserialize, Serialize};

/// A seller's business booking profile, including booking policy, appointment settings, etc.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BusinessBookingProfile {
    /// The ID of the seller, obtainable using the Merchants API.
    /// Max Length 32
    pub seller_id: Option<String>,
    /// **Read only** The RFC 3339 timestamp specifying the booking's creation time.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time: UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<DateTime>,
    /// Indicates whether the seller is open for booking.
    pub booking_enabled: Option<bool>,
    /// The choice of customer's time zone information of a booking. The Square online booking site
    /// and all notifications to customers uses either the seller location’s time zone or the time
    /// zone the customer chooses at booking.
    pub customer_timezone_choice: Option<BusinessBookingProfileCustomerTimezoneChoice>,
    /// The policy for the seller to automatically accept booking requests (AcceptAll) or not
    /// (RequiresAcceptance).
    pub booking_policy: Option<BusinessBookingProfileBookingPolicy>,
    /// Indicates whether customers can cancel or reschedule their own bookings (true) or not
    /// (false).
    pub allow_user_cancel: Option<bool>,
    /// Settings for appointment-type bookings.
    pub business_appointment_settings: Option<BusinessAppointmentSettings>,
    /// Indicates whether the seller's subscription to Square Appointments supports creating,
    /// updating or canceling an appointment through the API (true) or not (false) using seller
    /// permission.
    pub support_seller_level_writes: Option<bool>,
}
