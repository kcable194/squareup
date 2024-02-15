//! Model for Locale enum

use serde::{Deserialize, Serialize};

/// Used in OAuth api. The locale to present the permission request form in. Square detects the appropriate
/// locale automatically. Only provide this value if the application can definitively determine
/// the preferred locale.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Locale {
    /// English - Ireland
    #[serde(rename = "en-IE")]
    EnIe,
    /// English - United States
    #[serde(rename = "en-US")]
    EnUs,
    /// English - Canada
    #[serde(rename = "en-CA")]
    EnCa,
    /// Spanish - United States
    #[serde(rename = "es-US")]
    EsUs,
    /// French - Canada
    #[serde(rename = "fr-CA")]
    FrCa,
    /// Japanese
    #[serde(rename = "ja-JP")]
    JaJp,
}
