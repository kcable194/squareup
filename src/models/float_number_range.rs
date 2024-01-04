//! Model struct for FloatNumberRange type

use serde::Serialize;

/// Specifies a decimal number range.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct FloatNumberRange {
    /// A decimal value indicating where the range starts.
    pub start_at: Option<String>,
    /// A decimal value indicating where the range ends.
    pub send_at: Option<String>,
}