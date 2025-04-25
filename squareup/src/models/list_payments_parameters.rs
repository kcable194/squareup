//! Model struct for ListPaymentsParameters (query parameters)

use super::{DateTime, enums::SortOrder};
use crate::models::enums::{CardBrand, PaymentSortField};
use std::fmt::Display;

/// This is a model struct for ListPaymentsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListPaymentsParameters {
    /// The timestamp for the beginning of the reporting period. Inclusive.
    ///
    /// Default: The current time minus one year.
    pub begin_time: Option<DateTime>,
    /// The timestamp for the end of the reporting period.
    ///
    /// Default: The current time.
    pub end_time: Option<DateTime>,
    /// The order in which results are listed.
    /// * `ASC` - Oldest to newest.
    /// * `DESC` - Newest to oldest (default).
    pub sort_order: Option<SortOrder>,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for the original query.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// Limit results to the location supplied. By default, results are returned for the default
    /// (main) location associated with the seller.
    pub location_id: Option<String>,
    /// The exact amount in the `total_money` for a payment.
    pub total: Option<i32>,
    /// The last four digits of a payment card.
    pub last_4: Option<String>,
    /// The brand of the payment card (for example, VISA).
    pub card_brand: Option<CardBrand>,
    /// The maximum number of results to be returned in a single page. It is possible to receive
    /// fewer results than the specified limit on a given page.
    ///
    /// The default value of 100 is also the maximum allowed value. If the provided value is greater
    /// than 100, it is ignored and the default value is used instead.
    ///
    /// Default: `100`
    pub limit: Option<i32>,
    /// Whether the payment was taken offline or not.
    pub is_offline_payment: Option<bool>,
    /// Indicates the start of the time range for which to retrieve offline payments, in RFC 3339
    /// format for timestamps. The range is determined using the
    /// offline_payment_details.client_created_at field for each Payment. If set, payments without
    /// a value set in offline_payment_details.client_created_at will not be returned.
    ///
    /// Default: The current time.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    //
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub offline_begin_time: Option<DateTime>,
    /// Indicates the end of the time range for which to retrieve offline payments, in RFC 3339
    /// format for timestamps. The range is determined using the
    /// offline_payment_details.client_created_at field for each Payment. If set, payments without
    /// a value set in offline_payment_details.client_created_at will not be returned.
    ///
    /// Default: The current time.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub offline_end_time: Option<DateTime>,
    /// Indicates the start of the time range to retrieve payments for, in RFC 3339 format. The
    /// range is determined using the updated_at field for each Payment.
    pub updated_at_begin_time: Option<DateTime>,
    /// Indicates the end of the time range to retrieve payments for, in RFC 3339 format. The range
    /// is determined using the updated_at field for each Payment.
    pub updated_at_end_time: Option<DateTime>,
    /// The field used to sort results by. The default is CREATED_AT.
    pub sort_field: Option<PaymentSortField>,
}

impl ListPaymentsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListPaymentsParameters> for String {
    fn from(list_payments_parameters: ListPaymentsParameters) -> Self {
        list_payments_parameters.to_string()
    }
}

impl Display for ListPaymentsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(begin_time) = &self.begin_time {
            params.push(format!("begin_time={}", begin_time));
        }

        if let Some(end_time) = &self.end_time {
            params.push(format!("end_time={}", end_time));
        }

        if let Some(sort_order) = &self.sort_order {
            params.push(format!(
                "sort_order={}",
                serde_json::to_string(sort_order).unwrap()
            ));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(location_id) = &self.location_id {
            params.push(format!("location_id={}", location_id));
        }

        if let Some(total) = &self.total {
            params.push(format!("total={}", total));
        }

        if let Some(last_4) = &self.last_4 {
            params.push(format!("last_4={}", last_4));
        }

        if let Some(card_brand) = &self.card_brand {
            params.push(format!("card_brand={:?}", card_brand));
        }

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(is_offline_payment) = &self.is_offline_payment {
            params.push(format!("is_offline_payment={}", is_offline_payment));
        }

        if let Some(offline_begin_time) = &self.offline_begin_time {
            params.push(format!("offline_begin_time={}", offline_begin_time));
        }

        if let Some(offline_end_time) = &self.offline_end_time {
            params.push(format!("offline_end_time={}", offline_end_time));
        }

        if let Some(updated_at_begin_time) = &self.updated_at_begin_time {
            params.push(format!("updated_at_begin_time={}", updated_at_begin_time));
        }

        if let Some(updated_at_end_time) = &self.updated_at_end_time {
            params.push(format!("updated_at_end_time={}", updated_at_end_time));
        }

        if let Some(sort_field) = &self.sort_field {
            params.push(format!("sort_field={}", sort_field));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
