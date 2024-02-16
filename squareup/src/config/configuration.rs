//! App configuration for the library

use super::Environment;
use crate::config::base_uri::BaseUri;
use crate::http::client::HttpClientConfiguration;

/// Configuration struct for the library
#[derive(Clone, Debug, Default)]
pub struct Configuration {
    /// Current API environment
    pub environment: Environment,
    /// Http Client Configuration instance.
    pub http_client_config: HttpClientConfiguration,
    /// Base URI
    pub base_uri: BaseUri,
}

impl Configuration {
    /// Gets the base Square API URL for the configured environment, including the API version
    /// specifier (e.g. "/v2")
    pub(crate) fn get_base_url(&self) -> String {
        let base_url = self.environment.get_base_url();
        format!("{}{}", base_url, self.base_uri.get_base_uri())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{BaseUri, Configuration, Environment};

    #[test]
    fn get_base_url_default_url() {
        let configuration = Configuration::default();
        assert_eq!(
            "https://connect.squareupsandbox.com/v2",
            configuration.get_base_url()
        );
    }

    #[test]
    fn get_base_url_with_different_base_uri() {
        let mut configuration = Configuration::default();
        configuration.base_uri = BaseUri::Custom("/custom_base_uri".to_string());
        assert_eq!(
            "https://connect.squareupsandbox.com/custom_base_uri",
            configuration.get_base_url()
        );
    }
}
