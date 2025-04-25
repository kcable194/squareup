//! Model struct for CatalogItem type.

use serde::{Deserialize, Serialize};

use super::{
    CatalogEcomSeoData, CatalogItemFoodAndBeverageDetails, CatalogItemModifierListInfo,
    CatalogItemOptionForItem, CatalogObject, CatalogObjectCategory, enums::CatalogItemProductType,
};

/// A [CatalogObject] instance of the `ITEM` type, also referred to as an item, in the catalog.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItem {
    /// The item's name. This is a searchable attribute for use in applicable query filters, its
    /// value must not be empty, and the length is of Unicode code points.
    pub name: Option<String>,
    /// The text of the item's display label in the Square Point of Sale app. Only up to the first
    /// five characters of the string are used. This attribute is searchable, and its value length
    /// is of Unicode code points.
    pub abbreviation: Option<String>,
    /// The color of the item's display label in the Square Point of Sale app. This must be a valid
    /// hex color code.
    pub label_color: Option<String>,
    /// Indicates whether the item is taxable (true) or non-taxable (false). Default is true.
    pub is_taxable: Option<bool>,
    /// If `true`, the item can be added to shipping orders from the merchant's online store.
    pub available_online: Option<bool>,
    /// If `true`, the item can be added to pickup orders from the merchant's online store.
    pub available_for_pickup: Option<bool>,
    /// If `true`, the item can be added to electronically fulfilled orders from the merchant's
    /// online store.
    pub available_electronically: Option<bool>,
    /// A set of IDs indicating the taxes enabled for this item. When updating an item, any taxes
    /// listed here will be added to the item. Taxes may also be added to or deleted from an item
    /// using `UpdateItemTaxes`.
    pub tax_ids: Option<Vec<String>>,
    /// A set of `CatalogItemModifierListInfo` objects representing the modifier lists that apply to
    /// this item, along with the overrides and min and max limits that are specific to this item.
    /// Modifier lists may also be added to or deleted from an item using `UpdateItemModifierLists`.
    pub modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
    /// A list of [CatalogItemVariation] objects for this item. An item must have at least one
    /// variation.
    pub variations: Option<Vec<CatalogObject>>,
    /// The product type of the item. May not be changed once an item has been created.
    ///
    /// Only items of product type `REGULAR` or `APPOINTMENTS_SERVICE` may be created by this API;
    /// items with other product types are read-only.
    pub product_type: Option<CatalogItemProductType>,
    /// If `false`, the Square Point of Sale app will present the `CatalogItem`'s details screen
    /// immediately, allowing the merchant to choose `CatalogModifier`s before adding the item to
    /// the cart. This is the default behavior.
    ///
    /// If `true`, the Square Point of Sale app will immediately add the item to the cart with the
    /// pre-selected modifiers, and merchants can edit modifiers by drilling down onto the item's
    /// details.
    ///
    /// Third-party clients are encouraged to implement similar behaviors.
    pub skip_modifier_screen: Option<bool>,
    /// List of item options IDs for this item. Used to manage and group item variations in a
    /// specified order.
    ///
    /// Maximum: 6 item options.
    pub item_options: Option<Vec<CatalogItemOptionForItem>>,
    /// The IDs of images associated with this `CatalogItem` instance. These images will be shown to
    /// customers in Square Online Store. The first image will show up as the icon for this item in
    /// POS.
    pub image_ids: Option<Vec<String>>,
    /// A name to sort the item by. If this name is unspecified, namely, the `sort_name` field is
    /// absent, the regular `name` field is used for sorting.
    ///
    /// It is currently supported for sellers of the Japanese locale only.
    pub sort_name: Option<String>,
    /// The list of categories.
    pub categories: Option<Vec<CatalogObjectCategory>>,
    /// The item's description as expressed in valid HTML elements. The length of this field value, including those of
    /// HTML tags, is of Unicode points. With application query filters, the text values of the HTML
    /// elements and attributes are searchable. Invalid or unsupported HTML elements or attributes are
    /// ignored.
    /// Max Length = 65535
    pub description_html: Option<String>,
    /// **Read only** A server-generated plaintext version of the description_html field, without formatting tags.
    pub description_plaintext: Option<String>,
    /// A list of IDs representing channels, such as a Square Online site, where the item can be made visible or
    /// available.
    pub channels: Option<Vec<String>>,
    /// Indicates whether this item is archived (true) or not (false).
    pub is_archived: Option<bool>,
    /// The SEO data for a seller's Square Online store.
    pub ecom_seo_data: Option<CatalogEcomSeoData>,
    /// The food and beverage-specific details for the FoodAndBev item.
    pub food_and_beverage_details: Option<CatalogItemFoodAndBeverageDetails>,
    /// The item's reporting category.
    pub reporting_category: Option<CatalogObjectCategory>,
}
