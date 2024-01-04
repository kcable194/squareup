//! Model struct for EmptyRequestBody type

use super::{Address, CustomerTaxIds};
use serde::Serialize;
/// This is a model struct for EmptyRequestBody type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct EmptyRequestBody {}