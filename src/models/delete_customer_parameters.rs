//! Model struct for DeleteCustomerParameters (query parameters)

use std::fmt::Display;

/// This is a model struct for DeleteCustomerParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct DeleteCustomerParameters {
    /// The current version of the customer profile.
    ///
    /// As a best practice, you should include this parameter to enable optimistic concurrency
    /// control. For more information, see Delete a customer profile.
    pub version: Option<i64>,
}

impl DeleteCustomerParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<DeleteCustomerParameters> for String {
    fn from(delete_customer_parameters: DeleteCustomerParameters) -> Self {
        delete_customer_parameters.to_string()
    }
}

impl Display for DeleteCustomerParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if self.version.is_some() {
            params.push(format!("version={}", self.version.unwrap()));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}