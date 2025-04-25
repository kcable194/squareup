//! Model struct for CalculateOrderResponse type

use serde::Deserialize;

use super::{Order, errors::Error};

/// This is a model struct for CalculateOrderResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CalculateOrderResponse {
    /// The calculated version of the order provided in the request.
    pub order: Order,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
