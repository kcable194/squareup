//! Model struct for Range type

use serde::Serialize;

/// This is a model struct for Range type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Range {
    /// The lower bound of the number range. At least one of min or max must be specified.
    /// If unspecified, the results will have no minimum value.
    pub min: Option<String>,
    /// The upper bound of the number range. At least one of min or max must be specified.
    /// If unspecified, the results will have no maximum value.
    pub max: Option<String>,
}
