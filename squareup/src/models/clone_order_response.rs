//! Model struct for CloneOrderResponse type

use serde::Deserialize;

use super::{Order, errors::Error};

/// This is a model struct for CloneOrderResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CloneOrderResponse {
    /// The cloned order.
    pub order: Order,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
