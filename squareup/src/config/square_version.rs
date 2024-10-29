//! Square API version management, will correspond to crate version

pub const CURRENT_SQUARE_VERSION: &str = "2024-10-17";

/// Identifies the Square api version
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SquareVersion {
    /// Current api version that coincides with crate version
    SquareVersion,
    /// No guarantees that this will work, but adding as option
    CustomSquareVersion(String),
}

impl SquareVersion {
    /// Returns the correct api version as a String
    pub fn as_string(&self) -> String {
        match self {
            SquareVersion::SquareVersion => String::from(CURRENT_SQUARE_VERSION),
            SquareVersion::CustomSquareVersion(custom_version) => custom_version.clone(),
        }
    }
}

impl Default for SquareVersion {
    /// Default to current api version
    fn default() -> Self {
        Self::SquareVersion
    }
}

#[cfg(test)]
mod tests {
    use crate::config::square_version::CURRENT_SQUARE_VERSION;
    use crate::config::SquareVersion;

    #[test]
    fn as_string_default() {
        let square_version = SquareVersion::default();
        assert_eq!(
            CURRENT_SQUARE_VERSION.to_string(),
            square_version.as_string()
        );
    }

    #[test]
    fn as_string_custom() {
        let custom_version = "2020-05-10".to_string();
        let square_version = SquareVersion::CustomSquareVersion(custom_version.clone());
        assert_eq!(custom_version, square_version.as_string());
    }

    #[test]
    fn default() {
        let square_version = SquareVersion::default();
        assert_eq!(SquareVersion::SquareVersion, square_version);
    }
}
