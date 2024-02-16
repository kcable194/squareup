//! Square environment in which to operate

use serde::{Deserialize, Serialize};

const PRODUCTION_URL: &str = "https://connect.squareup.com";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com";

/// Identifies the Square environment in which this app instance is operating
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Environment {
    Production,
    Sandbox,
}

impl Environment {
    /// Gets the base Square API URL for this environment
    pub fn get_base_url(&self) -> String {
        match self {
            Environment::Production => String::from(PRODUCTION_URL),
            Environment::Sandbox => String::from(SANDBOX_URL),
        }
    }
}

impl Default for Environment {
    /// default to env variable and if not present, default to Sandbox
    fn default() -> Self {
        let env_string = std::env::var("SQUARE_ENVIRONMENT").unwrap_or(String::from("SANDBOX"));
        return match env_string.as_str() {
            "PRODUCTION" => Self::Production,
            "SANDBOX" => Self::Sandbox,
            _ => Self::Sandbox,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Environment;

    #[test]
    fn get_base_url_default() {
        let environment = Environment::default();
        assert_eq!(Environment::Sandbox, environment);
        assert_eq!(
            String::from("https://connect.squareupsandbox.com"),
            environment.get_base_url()
        )
    }

    #[test]
    fn get_base_url_production() {
        assert_eq!(
            String::from("https://connect.squareup.com"),
            Environment::Production.get_base_url()
        )
    }

    #[test]
    fn get_base_url_sandbox() {
        assert_eq!(
            String::from("https://connect.squareupsandbox.com"),
            Environment::Sandbox.get_base_url()
        )
    }
}
