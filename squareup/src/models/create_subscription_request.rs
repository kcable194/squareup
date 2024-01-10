//! Request body struct for the Create Subscription API

use serde::Serialize;

use super::{enums::Timezone, Money, Phase, SubscriptionSource};

/// This is a model class for CreateSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateSubscriptionRequest {
    /// A unique string that identifies this CreateSubscription request. If you do not provide a unique
    /// string (or provide an empty string as the value), the endpoint treats each request as independent.
    pub idempotency_key: Option<String>,
    /// The ID of the location the subscription is associated with.
    pub location_id: String,
    /// The ID of the subscription plan variation created using the Catalog API.
    pub plan_variation_id: Option<String>,
    /// The ID of the customer subscribing to the subscription plan variation.
    pub customer_id: String,
    /// The YYYY-MM-DD-formatted date to start the subscription. If it is unspecified,
    /// the subscription starts immediately.
    pub start_date: Option<String>,
    /// The YYYY-MM-DD-formatted date when the newly created subscription is scheduled for cancellation.
    ///
    /// This date overrides the cancellation date set in the plan variation configuration. If the
    /// cancellation date is earlier than the end date of a subscription cycle, the subscription stops at
    /// the canceled date and the subscriber is sent a prorated invoice at the beginning of the canceled
    /// cycle.
    ///
    /// When the subscription plan of the newly created subscription has a fixed number of cycles and the
    /// canceled_date occurs before the subscription plan expires, the specified canceled_date sets the
    /// date when the subscription stops through the end of the last cycle.
    pub canceled_date: Option<String>,
    /// The tax to add when billing the subscription. The percentage is expressed in decimal form,
    /// using a '.' as the decimal separator and without a '%' sign. For example, a value of 7.5
    /// corresponds to 7.5%.
    /// Max Length 10
    pub tax_percentage: Option<String>,
    /// A custom price to apply for the subscription. If specified, it overrides the price configured by
    /// the subscription plan.
    pub price_override_money: Option<Money>,
    /// The ID of the subscriber's card to charge. If it is not specified, the subscriber receives an
    /// invoice via email with a link to pay for their subscription.
    pub card_id: Option<String>,
    /// The timezone that is used in date calculations for the subscription. If unset, defaults to the
    /// location timezone. If a timezone is not configured for the location, defaults to "America/New_York".
    /// Format: the IANA Timezone Database identifier for the location timezone.
    pub timezone: Option<Timezone>,
    /// The origination details of the subscription.
    pub source: Option<SubscriptionSource>,
    /// The day-of-the-month to change the billing date to.
    /// Min 1, Max 31
    pub monthly_billing_anchor_date: Option<i32>,
    /// array of phases for this subscription
    pub phases: Option<Vec<Phase>>,
}
