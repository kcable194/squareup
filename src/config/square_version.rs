//! Square API version management, will correspond to crate version

pub(crate) const ORIGINAL_SQUARE_VERSION: &str = "2022-02-16";
pub(crate) const CURRENT_SQUARE_VERSION: &str = "2023-12-13";

/// Identifies the Square api version
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SquareVersion {
    /// Original version was version when forked
    OriginalSquareVersion,
    /// Current api version that coincides with crate version
    CurrentSquareVersion,
    /// No guarantees that this will work, but adding as option
    CustomSquareVersion(String),
}

impl SquareVersion {
    /// Returns the correct api version as a String
    pub fn as_string(&self) -> String {
        match self {
            SquareVersion::OriginalSquareVersion => String::from(ORIGINAL_SQUARE_VERSION),
            SquareVersion::CurrentSquareVersion => String::from(CURRENT_SQUARE_VERSION),
            SquareVersion::CustomSquareVersion(custom_version) => custom_version.clone(),
        }
    }
}

impl Default for SquareVersion {
    /// Default to current api version
    fn default() -> Self {
        Self::CurrentSquareVersion
    }
}

#[cfg(test)]
mod tests {
    use crate::config::square_version::{CURRENT_SQUARE_VERSION, ORIGINAL_SQUARE_VERSION};
    use crate::config::SquareVersion;

    #[test]
    fn as_string_default() {
        let square_version = SquareVersion::default();
        assert_eq!(CURRENT_SQUARE_VERSION.to_string(), square_version.as_string());
    }

    #[test]
    fn as_string_original() {
        let square_version = SquareVersion::OriginalSquareVersion;
        assert_eq!(ORIGINAL_SQUARE_VERSION.to_string(), square_version.as_string());
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
        assert_eq!(SquareVersion::CurrentSquareVersion, square_version);
    }
}