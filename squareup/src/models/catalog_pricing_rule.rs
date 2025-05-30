//! Model struct for CatalogPricingRule type.

use serde::{Deserialize, Serialize};

use super::{Money, enums::ExcludeStrategy};

/// Defines how discounts are automatically applied to a set of items that match the pricing rule
/// during the active time period.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogPricingRule {
    /// User-defined name for the pricing rule. For example, "Buy one get one free" or "10% off".
    pub name: Option<String>,
    /// A list of unique IDs for the catalog time periods when this pricing rule is in effect. If
    /// left unset, the pricing rule is always in effect.
    pub time_period_ids: Option<Vec<String>>,
    /// Unique ID for the `CatalogDiscount` to take off the price of all matched items.
    pub discount_id: Option<String>,
    /// Unique ID for the `CatalogProductSet` that will be matched by this rule. A match rule
    /// matches within the entire cart, and can match multiple times. This field will always be set.
    pub match_products_id: Option<String>,
    /// **Deprecated:** Please use the `exclude_products_id` field to apply an exclude set instead.
    /// Exclude sets allow better control over quantity ranges and offer more flexibility for which
    /// matched items receive a discount.
    ///
    /// `CatalogProductSet` to apply the pricing to. An apply rule matches within the subset of the
    /// cart that fits the match rules (the match set). An apply rule can only match once in the
    /// match set. If not supplied, the pricing will be applied to all products in the match set.
    /// Other products retain their base price, or a price generated by other rules.
    #[deprecated]
    pub apply_products_id: Option<String>,
    /// `CatalogProductSet` to exclude from the pricing rule. An exclude rule matches within the
    /// subset of the cart that fits the match rules (the match set). An exclude rule can only match
    /// once in the match set. If not supplied, the pricing will be applied to all products in the
    /// match set. Other products retain their base price, or a price generated by other rules.
    pub exclude_products_id: Option<String>,
    /// Represents the date the Pricing Rule is valid from. Represented in RFC 3339 full-date format
    /// (YYYY-MM-DD).
    pub valid_from_date: Option<String>,
    /// Represents the local time the pricing rule should be valid from. Represented in RFC 3339
    /// partial-time format (HH:MM:SS). Partial seconds will be truncated.
    pub valid_from_local_time: Option<String>,
    /// Represents the date the Pricing Rule is valid until. Represented in RFC 3339 full-date
    /// format (YYYY-MM-DD).
    pub valid_until_date: Option<String>,
    /// Represents the local time the pricing rule should be valid until. Represented in RFC 3339
    /// partial-time format (HH:MM:SS). Partial seconds will be truncated.
    pub valid_until_local_time: Option<String>,
    /// If an `exclude_products_id` was given, controls which subset of matched products is excluded
    /// from any discounts.
    ///
    /// Default value: `LEAST_EXPENSIVE`
    pub exclude_strategy: Option<ExcludeStrategy>,
    /// The minimum order subtotal (before discounts or taxes are applied) that must be met before
    /// this rule may be applied.
    pub minimum_order_subtotal_money: Option<Money>,
    /// A list of IDs of customer groups, the members of which are eligible for discounts specified
    /// in this pricing rule. Notice that a group ID is generated by the Customers API. If this
    /// field is not set, the specified discount applies to matched products sold to anyone whether
    /// the buyer has a customer profile created or not. If this `customer_group_ids_any` field is
    /// set, the specified discount applies only to matched products sold to customers belonging to
    /// the specified customer groups.
    pub customer_group_ids_any: Option<Vec<String>>,
}
