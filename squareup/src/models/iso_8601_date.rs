//! Model struct for Iso8601Date type

use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serializer};
use std::fmt;

/// ISO 8601 Datetime
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Iso8601Date {
    pub year: String,
    pub month: String,
    pub date: String,
}

// Implement Serialize manually for Iso8601Date
impl serde::Serialize for Iso8601Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Concatenate the year, month, and date with dashes
        let formatted_date = format!("{}-{}-{}", self.year, self.month, self.date);

        // Serialize the string directly
        serializer.serialize_str(&formatted_date)
    }
}

// Implementing the Deserialize manually
impl<'de> Deserialize<'de> for Iso8601Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Iso8601DateVisitor;

        impl<'de> Visitor<'de> for Iso8601DateVisitor {
            type Value = Iso8601Date;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in ISO8601 date format")
            }

            fn visit_str<E>(self, value: &str) -> Result<Iso8601Date, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.split('-').collect();
                if parts.len() != 3 {
                    return Err(E::custom(format!("Invalid ISO8601 date: {}", value)));
                }
                Ok(Iso8601Date {
                    year: parts[0].to_string(),
                    month: parts[1].to_string(),
                    date: parts[2].to_string(),
                })
            }
        }

        deserializer.deserialize_str(Iso8601DateVisitor)
    }
}
