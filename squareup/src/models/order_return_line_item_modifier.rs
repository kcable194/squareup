//! Model struct for OrderReturnLineItemModifier type

use serde::{Deserialize, Serialize};

use super::Money;

/// A line item modifier being returned.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnLineItemModifier {
    /// A unique ID that identifies the return modifier only within this order.
    pub uid: Option<String>,
    /// The modifier `uid` from the order's line item that contains the original sale of this line
    /// item modifier.
    pub source_modifier_uid: Option<String>,
    /// The catalog object ID referencing [CatalogModifier].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this line item modifier references.
    pub catalog_version: Option<i64>,
    /// The name of the item modifier.
    pub name: Option<String>,
    /// The base price for the modifier.
    ///
    /// `base_price_money` is required for ad hoc modifiers. If both `catalog_object_id` and
    /// `base_price_money` are set, `base_price_money` overrides the predefined [CatalogModifier]
    /// price.
    pub base_price_money: Option<Money>,
    /// **Read only** The total price of the item modifier for its line item. This is the modifier's
    /// `base_price_money` multiplied by the line item's quantity.
    pub total_price_money: Option<Money>,
    /// The quantity of the line item modifier. The modifier quantity can be 0 or more. For example, suppose
    /// a restaurant offers a cheeseburger on the menu. When a buyer orders this item, the restaurant records
    /// the purchase by creating an Order object with a line item for a burger. The line item includes a line
    /// item modifier: the name is cheese and the quantity is 1. The buyer has the option to order extra
    /// cheese (or no cheese). If the buyer chooses the extra cheese option, the modifier quantity increases
    /// to 2. If the buyer does not want any cheese, the modifier quantity is set to 0.
    pub quantity: Option<String>,
}
