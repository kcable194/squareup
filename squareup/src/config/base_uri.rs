//! Square major api version to use

const V2_URI: &str = "/v2";

/// Identifies square major api version via base uri
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BaseUri {
    V2,
    Custom(String),
}

impl BaseUri {
    /// Gets the base Square API URL for this environment
    pub fn get_base_uri(&self) -> String {
        match self {
            BaseUri::V2 => String::from(V2_URI),
            BaseUri::Custom(custom_uri) => custom_uri.to_owned(),
        }
    }
}

impl Default for BaseUri {
    /// Default is the default uri
    fn default() -> Self {
        Self::V2
    }
}
