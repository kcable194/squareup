//! Model struct for Address type

use serde::{Deserialize, Serialize};

use super::enums::Country;

/// Represents a postal address in a country.
///
/// For more information, see [Working with
/// Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Address {
    /// The first line of the address.
    ///
    /// Fields that start with `address_line` provide the address's most specific details, like
    /// street number, street name, and building name. They do not provide less specific details
    /// like city, state/province, or country (these details are provided in other fields).
    pub address_line_1: Option<String>,
    /// The second line of the address, if any.
    pub address_line_2: Option<String>,
    /// The third line of the address, if any.
    pub address_line_3: Option<String>,
    /// The city or town of the address. For a full list of field meanings by country, see [Working
    /// with Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
    pub locality: Option<String>,
    /// A civil region within the address's `locality`, if any.
    pub sublocality: Option<String>,
    /// A civil region within the address's `sublocality`, if any.
    pub sublocality_2: Option<String>,
    /// A civil region within the address's `sublocality_2`, if any.
    pub sublocality_3: Option<String>,
    /// A civil entity within the address's country. In the US, this is the state. For a full list
    /// of field meanings by country, see [Working with
    /// Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
    pub administrative_district_level_1: Option<String>,
    /// A civil entity within the address's administrative_district_level_1. In the US, this is the county.
    pub administrative_district_level_2: Option<String>,
    /// A civil entity within the address's administrative_district_level_2, if any.
    pub administrative_district_level_3: Option<String>,
    /// The address's postal code. For a full list of field meanings by country, see [Working with
    /// Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
    /// Can be specified in ZIP+4 format (for example, 12345-6789).
    pub postal_code: Option<String>,
    /// The address's country, in the two-letter format of ISO 3166. For example, `US` or `FR`.
    pub country: Option<Country>,
    /// Optional first name when it's representing recipient.
    pub first_name: Option<String>,
    /// Optional last name when it's representing recipient.
    pub last_name: Option<String>,
}
