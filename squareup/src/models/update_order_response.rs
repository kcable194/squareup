//! Model struct for UpdateOrderResponse type

use serde::Deserialize;

use super::{Order, errors::Error};

/// This is a model struct for UpdateOrderResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateOrderResponse {
    /// The updated order.
    pub order: Order,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
