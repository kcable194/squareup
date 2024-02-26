//! Response struct for the Create Payment Link API

use serde::Deserialize;

use super::{CatalogObject, Order};

/// This is a model struct for PaymentLinkRelatedResources type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct PaymentLinkRelatedResources {
    /// The order associated with the payment link.
    pub orders: Option<Vec<Order>>,
    /// The subscription plan associated with the payment link.
    pub subscription_plans: Option<Vec<CatalogObject>>,
}
