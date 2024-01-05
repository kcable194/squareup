//! Model struct for InvoiceAcceptedPaymentMethods type.

use serde::{Deserialize, Serialize};

/// The payment methods that customers can use to pay an invoice on the Square-hosted invoice page.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoiceAcceptedPaymentMethods {
    /// Indicates whether credit card or debit card payments are accepted. The default value is
    /// `false`.
    pub card: Option<bool>,
    /// Indicates whether Square gift card payments are accepted. The default value is `false`.
    pub square_gift_card: Option<bool>,
    /// Indicates whether bank transfer payments are accepted. The default value is `false`.
    ///
    /// This option is allowed only for invoices that have a single payment request of type
    /// `BALANCE`.
    pub bank_account: Option<bool>,
    /// Indicates whether Afterpay (also known as Clearpay) payments are accepted. The default value is false.
    ///
    /// This option is allowed only for invoices that have a single payment request of the BALANCE type. This
    /// payment method is supported if the seller account accepts Afterpay payments and the seller location
    /// is in a country where Afterpay invoice payments are supported. As a best practice, consider enabling
    /// an additional payment method when allowing buy_now_pay_later payments. For more information,
    /// including detailed requirements and processing limits, see Buy Now Pay Later payments with Afterpay.
    pub buy_now_pay_later: Option<bool>,
    /// Indicates whether Cash App payments are accepted. The default value is false.
    ///
    /// This payment method is supported only for seller locations in the United States.
    pub cash_app_pay: Option<bool>,
}
