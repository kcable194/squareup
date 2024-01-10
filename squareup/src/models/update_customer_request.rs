//! Model struct for UpdateCustomerRequest type

use super::{Address, CustomerTaxIds};
use serde::Serialize;
/// This is a model struct for UpdateCustomerRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateCustomerRequest {
    /// The given name (that is, the first name) associated with the customer profile.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the customer profile.
    pub family_name: Option<String>,
    /// A business name associated with the customer profile. The maximum length for this value
    /// is 500 characters.
    pub company_name: Option<String>,
    /// A nickname for the customer profile. The maximum length for this value is 100 characters.
    pub nickname: Option<String>,
    /// The email address associated with the customer profile.
    pub email_address: Option<String>,
    /// Represents a postal address in a country. For more information, see [Working with
    /// Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
    pub address: Option<Address>,
    /// The 11-digit phone number associated with the customer profile.
    pub phone_number: Option<String>,
    /// An optional second ID used to associate the customer profile with an entity in another system.
    /// The maximum length for this value is 100 characters.
    pub reference_id: Option<String>,
    /// A custom note associated with the customer profile.
    pub note: Option<String>,
    /// The birthday associated with the customer profile, in RFC 3339 format. The year is optional.
    /// The timezone and time are not allowed. For example, `0000-09-21T00:00:00-00:00` represents a
    /// birthday on September 21 and `1998-09-21T00:00:00-00:00` represents a birthday on September
    /// 21, 1998. You can also specify this value in `YYYY-MM-DD` format.
    pub birthday: Option<String>,
    /// The current version of the customer profile. As a best practice, you should include this field to
    /// enable optimistic concurrency control. For more information, see Update a customer profile.
    pub version: Option<i64>,
    /// Represents the tax ID associated with a [customer profile]($m/Customer). The corresponding
    /// `tax_ids` field is available only for customers of sellers in EU countries or the United
    /// Kingdom. For more information, see [Customer tax
    /// IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
    pub tax_ids: Option<CustomerTaxIds>,
}
