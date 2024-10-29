//! Model struct for RefundPaymentRequest type

use super::{DestinationDetailsExternalRefundDetails, Money};
use crate::models::destination_details_cash_refund_details::DestinationDetailsCashRefundDetails;
use serde::Serialize;

/// This is a model struct for RefundPaymentRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RefundPaymentRequest {
    /// A unique string that identifies this `RefundPayment` request. The key can be any valid
    /// string but must be unique for every `RefundPayment` request.
    ///
    /// Keys are limited to a max of 45 characters - however, the number of allowed characters might
    /// be less than 45, if multi-byte characters are used.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency).
    pub idempotency_key: String,
    /// The amount of money to refund.
    ///
    /// This amount cannot be more than the `total_money` value of the payment minus the total
    /// amount of all previously completed refunds for this payment.
    ///
    /// This amount must be specified in the smallest denomination of the applicable currency (for
    /// example, US dollar amounts are specified in cents). For more information, see [Working with
    /// Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts).
    ///
    /// The currency code must match the currency associated with the business that is charging the card.
    pub amount_money: Money,
    /// The amount of money the developer contributes to help cover the refunded amount. This amount
    /// is specified in the smallest denomination of the applicable currency (for example, US dollar
    /// amounts are specified in cents).
    ///
    /// The value cannot be more than the `amount_money`.
    ///
    /// You can specify this parameter in a refund request only if the same parameter was also
    /// included when taking the payment. This is part of the application fee scenario the API
    /// supports. For more information, see [Take Payments and Collect
    /// Fees](https://developer.squareup.com/docs/payments-api/take-payments-and-collect-fees).
    ///
    /// To set this field, `PaymentsWriteAdditionalRecipients` OAuth permission is required. For
    /// more information, see
    /// [Permissions](https://developer.squareup.com/docs/payments-api/take-payments-and-collect-fees#permissions).
    pub app_fee_money: Option<Money>,
    /// The unique ID of the payment being refunded. Required when unlinked=false, otherwise must not be set.
    pub payment_id: Option<String>,
    /// The ID indicating where funds will be refunded to. Required for unlinked refunds. For more
    /// information, see Create an unlinked refund [https://developer.squareup.com/docs/payments-api/refund-payments#create-an-unlinked-refund]
    ///
    /// For refunds linked to Square payments, destination_id is usually omitted; in this case,
    /// funds will be returned to the original payment source. The field may be specified in order
    /// to request a cross-method refund to a gift card. For more information, see Cross-method
    /// refunds to gift cards [https://developer.squareup.com/docs/payments-api/refund-payments#cross-method-refunds-to-gift-cards]
    pub destination_id: Option<String>,
    /// Indicates that the refund is not linked to a Square payment. If set to true, destination_id and
    /// location_id must be supplied while payment_id must not be provided.
    pub unlinked: Option<bool>,
    /// The location ID associated with the unlinked refund. Required for requests specifying unlinked=true.
    /// Otherwise, if included when unlinked=false, will throw an error.
    /// Max Length 50
    pub location_id: Option<String>,
    /// The Customer ID of the customer associated with the refund. This is required if the destination_id
    /// refers to a card on file created using the Cards API. Only allowed when unlinked=true.
    pub customer_id: Option<String>,
    /// A description of the reason for the refund.
    ///
    /// Max Length 192
    pub reason: Option<String>,
    /// Used for optimistic concurrency. This opaque token identifies the current `Payment` version
    /// that the caller expects. If the server has a different version of the Payment, the update
    /// fails and a response with a VERSION_MISMATCH error is returned. If the versions match, or
    /// the field is not provided, the refund proceeds as normal.
    pub payment_version_token: Option<String>,
    /// An optional [TeamMember] ID to associate with this refund.
    pub team_member_id: Option<String>,
    /// Additional details required when recording a cash refund (destination_id is CASH).
    pub cash_details: Option<DestinationDetailsCashRefundDetails>,
    /// Additional details required when recording an external refund (destination_id is EXTERNAL).
    pub external_details: Option<DestinationDetailsExternalRefundDetails>,
}
