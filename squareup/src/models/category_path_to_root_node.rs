//! Model struct for CategoryPathToRootNode type.

use serde::{Deserialize, Serialize};

/// A node in the path from a retrieved category to its root node.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CategoryPathToRootNode {
    /// The category's ID.
    pub category_id: Option<String>,
    /// The category's name.
    pub category_name: Option<String>,
}
