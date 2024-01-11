//! Square major api version to use

const DEFAULT_URI: &str = "/v2";

/// Identifies square major api version via base uri
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BaseUri {
    Default,
    Custom(String),
}

impl BaseUri {
    /// Gets the base Square API URL for this environment
    pub fn get_base_uri(&self) -> String {
        match self {
            BaseUri::Default => String::from(DEFAULT_URI),
            BaseUri::Custom(custom_uri) => custom_uri.to_owned(),
        }
    }
}

impl Default for BaseUri {
    /// Default is the default uri
    fn default() -> Self {
        Self::Default
    }
}
