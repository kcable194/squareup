//! Model struct for OrderLineItem type

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{enums::OrderLineItemItemType, Money, OrderLineItemAppliedDiscount, OrderLineItemAppliedServiceCharge, OrderLineItemAppliedTax, OrderLineItemModifier, OrderLineItemPricingBlocklists, OrderQuantityUnit};

/// Represents a line item in an order.
///
/// Each line item describes a different product to purchase, with its own quantity and price
/// details.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItem {
    /// A unique ID that identifies the line item only within this order.
    pub uid: Option<String>,
    /// The name of the line item.
    pub name: Option<String>,
    /// The quantity purchased, formatted as a decimal number. For example, "3".
    ///
    /// Line items with a quantity of "0" are automatically removed when paying for or otherwise
    /// completing the order.
    ///
    /// Line items with a `quantity_unit` can have non-integer quantities. For example, "1.70000".
    pub quantity: String,
    /// The unit and precision that this line item's quantity is measured in.
    pub quantity_unit: Option<OrderQuantityUnit>,
    /// The note of the line item.
    pub note: Option<String>,
    /// The [CatalogItemVariation] ID applied to this line item.
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this line item references.
    pub catalog_version: Option<i64>,
    /// The name of the variation applied to this line item.
    pub variation_name: Option<String>,
    /// The type of line item: an itemized sale, a non-itemized sale (custom amount), or the
    /// activation or reloading of a gift card.
    pub item_type: Option<OrderLineItemItemType>,
    /// Application-defined data attached to this order. Metadata fields are intended to store
    /// descriptive references or associations with an entity in another system or store brief
    /// information about the object. Square does not process this field; it only stores and returns
    /// it in relevant API calls. Do not use metadata to store any sensitive information (such as
    /// personally identifiable information or card details).
    ///
    /// Keys written by applications must be 60 characters or less and must be in the character set
    /// `[a-zA-Z0-9_-]`. Entries can also include metadata generated by Square. These keys are
    /// prefixed with a namespace, separated from the key with a ':' character.
    ///
    /// Values have a maximum length of 255 characters.
    ///
    /// An application can have up to 10 entries per metadata field.
    ///
    /// Entries written by applications are private and can only be read or modified by the same
    /// application.
    ///
    /// For more information,
    /// see [Metadata](https://developer.squareup.com/docs/build-basics/metadata).
    pub metadata: Option<HashMap<String, String>>,
    /// The [CatalogModifier]s applied to this line item.
    pub modifiers: Option<Vec<OrderLineItemModifier>>,
    /// The list of references to taxes applied to this line item. Each `OrderLineItemAppliedTax`
    /// has a `tax_uid` that references the `uid` of a top-level `OrderLineItemTax` applied to the
    /// line item. On reads, the amount applied is populated.
    ///
    /// An `OrderLineItemAppliedTax` is automatically created on every line item for all `ORDER`
    /// scoped taxes added to the order. `OrderLineItemAppliedTax` records for `LineItem` scoped
    /// taxes must be added in requests for the tax to apply to any line items.
    ///
    /// To change the amount of a tax, modify the referenced top-level tax.
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    /// The list of references to discounts applied to this line item. Each
    /// `OrderLineItemAppliedDiscount` has a `discount_uid` that references the `uid` of a top-level
    /// `OrderLineItemDiscounts` applied to the line item. On reads, the amount applied is
    /// populated.
    ///
    /// An `OrderLineItemAppliedDiscount` is automatically created on every line item for all
    /// `ORDER` scoped discounts that are added to the order. `OrderLineItemAppliedDiscount` records
    /// for `LineItem` scoped discounts must be added in requests for the discount to apply to any
    /// line items.
    ///
    /// To change the amount of a discount, modify the referenced top-level discount.
    pub applied_discounts: Option<Vec<OrderLineItemAppliedDiscount>>,
    /// The list of references to service charges applied to this line item. Each OrderLineItemAppliedServiceCharge
    /// has a service_charge_id that references the uid of a top-level OrderServiceCharge applied to the line item.
    /// On reads, the amount applied is populated.
    ///
    /// To change the amount of a service charge, modify the referenced top-level service charge.
    pub applied_service_charges: Option<Vec<OrderLineItemAppliedServiceCharge>>,
    /// The base price for a single unit of the line item.
    pub base_price_money: Option<Money>,
    /// **Read only** The total price of all item variations sold in this line item. The price is
    /// calculated as `base_price_money` multiplied by `quantity`. It does not include modifiers.
    pub variation_total_price_money: Option<Money>,
    /// **Read only** The amount of money made in gross sales for this line item. The amount is
    /// calculated as the sum of the variation's total price and each modifier's total price.
    pub gross_sales_money: Option<Money>,
    /// **Read only** The total amount of tax money to collect for the line item.
    pub total_tax_money: Option<Money>,
    /// **Read only** The total amount of discount money to collect for the line item.
    pub total_discount_money: Option<Money>,
    /// **Read only** The total amount of money to collect for this line item.
    pub total_money: Option<Money>,
    /// Describes pricing adjustments that are blocked from manual and automatic application to a
    /// line item. For more information, see [Apply Taxes and
    /// Discounts](https://developer.squareup.com/docs/orders-api/apply-taxes-and-discounts).
    pub pricing_blocklists: Option<OrderLineItemPricingBlocklists>,
    /// **Read only** The total amount of apportioned service charge money to collect for the line item.
    pub total_service_charge_money: Option<Money>,
}
