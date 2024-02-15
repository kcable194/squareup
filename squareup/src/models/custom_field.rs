use serde::{Deserialize, Serialize};

/// CustomField : Describes a custom form field to add to the checkout page to collect more information from buyers during checkout. For more information, see [Specify checkout options](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations#specify-checkout-options-1).

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
  /// The title of the custom field.
  #[serde(rename = "title")]
  title: String
}

impl CustomField {
  /// Describes a custom form field to add to the checkout page to collect more information from buyers during checkout. For more information, see [Specify checkout options](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations#specify-checkout-options-1).
  pub fn new(title: String) -> CustomField {
    CustomField {
      title
    }
  }

  pub fn set_title(&mut self, title: String) {
    self.title = title;
  }

  pub fn with_title(mut self, title: String) -> CustomField {
    self.title = title;
    self
  }

  pub fn title(&self) -> &String {
    &self.title
  }


}



