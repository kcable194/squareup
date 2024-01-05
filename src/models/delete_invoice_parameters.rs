//! Query parameters for the Delete Invoice API

use std::fmt::Display;

/// This is a model struct for DeleteInvoiceParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct DeleteInvoiceParameters {
    /// The version of the [Invoice] to delete. If you do not know the version, you can call
    /// [GetInvoice](https://developer.squareup.com/reference/square/invoices-api/get-invoice) or
    /// [ListInvoices](https://developer.squareup.com/reference/square/invoices-api/list-invoices).
    pub version: Option<i32>,
}

impl DeleteInvoiceParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<DeleteInvoiceParameters> for String {
    fn from(delete_invoice_parameters: DeleteInvoiceParameters) -> Self {
        delete_invoice_parameters.to_string()
    }
}

impl Display for DeleteInvoiceParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(version) = &self.version {
            params.push(format!("version={}", version));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
