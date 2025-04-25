//! Model struct for ListCustomersResponse type

use serde::{Deserialize, Serialize};

use super::{Customer, errors::Error};

/// This is a model struct for ListCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListCustomersResponse {
    /// Information on errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested list of `Customers`.
    pub customers: Option<Vec<Customer>>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the final
    /// response. See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
    /// The total count of customers associated with the Square account. Only customer profiles with public
    /// information (given_name, family_name, company_name, email_address, or phone_number) are counted.
    /// This field is present only if count is set to true in the request.
    pub count: Option<i64>,
}
