//! Response struct for the Test webhook subscription API

use crate::models::SubscriptionTestResult;
use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for TestWebhookSubscriptionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TestWebhookSubscriptionResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The SubscriptionTestResult object.
    pub subscription_test_result: Option<SubscriptionTestResult>,
}
