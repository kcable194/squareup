//! Query parameters for the List Webhook Subscriptions API

use crate::models::enums::SortOrder;
use std::fmt::Display;

/// This is a model struct for ListWebhookSubscriptionsParams (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListWebhookSubscriptionsParams {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve
    /// the next set of results for your original query.
    pub cursor: Option<String>,
    /// Includes disabled Subscriptions. By default, all enabled Subscriptions are returned.
    pub include_disabled: Option<bool>,
    /// Sorts the returned list by when the Subscription was created with the specified order.
    /// This field defaults to ASC.
    pub sort_order: Option<SortOrder>,
    /// The maximum number of results to be returned in a single page. It is possible to receive
    /// fewer results than the specified limit on a given page. The default value of 100 is also
    /// the maximum allowed value.
    ///
    /// Default: 100
    pub limit: Option<i32>,
}

impl ListWebhookSubscriptionsParams {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListWebhookSubscriptionsParams> for String {
    fn from(list_webhook_subscriptions_params: ListWebhookSubscriptionsParams) -> Self {
        list_webhook_subscriptions_params.to_string()
    }
}

impl Display for ListWebhookSubscriptionsParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(include_disabled) = &self.include_disabled {
            params.push(format!("include_disabled={}", include_disabled));
        }

        if let Some(sort_order) = &self.sort_order {
            params.push(format!("sort_order={:?}", sort_order));
        }

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
