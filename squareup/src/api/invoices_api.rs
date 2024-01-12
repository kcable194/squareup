//! Create and manage invoices.
//!
//! The Invoices API lets you create and manage invoices for orders that were created using the
//! Orders API. After you create the invoice and configure its delivery method, payment schedule,
//! and other invoice settings, you can publish the invoice. Depending on the invoice settings,
//! Square sends the invoice to the customer or automatically charges a card on file. Square also
//! hosts each invoice on a web page where customers can easily pay for it.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::SquareApiError, CancelInvoiceRequest, CancelInvoiceResponse, CreateInvoiceRequest,
        CreateInvoiceResponse, DeleteInvoiceParameters, DeleteInvoiceResponse, GetInvoiceResponse,
        ListInvoicesParameters, ListInvoicesResponse, PublishInvoiceRequest,
        PublishInvoiceResponse, SearchInvoicesRequest, SearchInvoicesResponse,
        UpdateInvoiceRequest, UpdateInvoiceResponse,
    },
    SquareClient,
};

const DEFAULT_URI: &str = "/invoices";

/// Create and manage invoices.
pub struct InvoicesApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Invoices API endpoints
    http_client: HttpClient,
}

impl InvoicesApi {
    /// Instantiates a new `InvoicesApi`
    pub fn new(square_client: SquareClient) -> InvoicesApi {
        InvoicesApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// Returns a list of invoices for a given location.
    ///
    /// The response is paginated. If truncated, the response includes a `cursor` that you use in a
    /// subsequent request to retrieve the next set of invoices.
    pub async fn list_invoices(
        &self,
        params: &ListInvoicesParameters,
    ) -> Result<ListInvoicesResponse, SquareApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a draft [Invoice] for an order created using the Orders API.
    ///
    /// A draft invoice remains in your account and no action is taken. You must publish the invoice
    /// before Square can process it (send it to the customer's email address or charge the
    /// customer’s card on file).
    pub async fn create_invoice(
        &self,
        body: &CreateInvoiceRequest,
    ) -> Result<CreateInvoiceResponse, SquareApiError> {
        let response = self.http_client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Searches for invoices from a location specified in the filter.
    ///
    /// You can optionally specify customers in the filter for whom to retrieve invoices. In the
    /// current implementation, you can only specify one location and optionally one customer.
    ///
    /// The response is paginated. If truncated, the response includes a `cursor` that you use in a
    /// subsequent request to retrieve the next set of invoices.
    pub async fn search_invoices(
        &self,
        body: &SearchInvoicesRequest,
    ) -> Result<SearchInvoicesResponse, SquareApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes the specified invoice.
    ///
    /// When an invoice is deleted, the associated order status changes to CANCELED. You can only
    /// delete a draft invoice (you cannot delete a published invoice, including one that is
    /// scheduled for processing).
    pub async fn delete_invoice(
        &self,
        invoice_id: &str,
        params: &DeleteInvoiceParameters,
    ) -> Result<DeleteInvoiceResponse, SquareApiError> {
        let url = format!("{}/{}{}", &self.url(), invoice_id, params.to_query_string());
        let response = self.http_client.delete(&url).await?;

        response.deserialize().await
    }

    /// Retrieves an invoice by invoice ID.
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<GetInvoiceResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), invoice_id);
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates an invoice by modifying fields, clearing fields, or both.
    ///
    /// For most updates, you can use a sparse [Invoice] object to add fields or change values and
    /// use the `fields_to_clear` field to specify fields to clear. However, some restrictions
    /// apply. For example, you cannot change the `order_id` or `location_id` field and you must
    /// provide the complete `custom_fields` list to update a custom field. Published invoices have
    /// additional restrictions.
    pub async fn update_invoice(
        &self,
        invoice_id: &str,
        body: &UpdateInvoiceRequest,
    ) -> Result<UpdateInvoiceResponse, SquareApiError> {
        let url = format!("{}/{}", &self.url(), invoice_id);
        let response = self.http_client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Cancels an invoice.
    ///
    /// The seller cannot collect payments for the canceled invoice.
    ///
    /// You cannot cancel an invoice in the `DRAFT` state or in a terminal state: `PAID`,
    /// `REFUNDED`, `CANCELED`, or `FAILED`.
    pub async fn cancel_invoice(
        &self,
        invoice_id: &str,
        body: &CancelInvoiceRequest,
    ) -> Result<CancelInvoiceResponse, SquareApiError> {
        let url = format!("{}/{}/cancel", &self.url(), invoice_id);
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Publishes the specified draft invoice.
    ///
    /// After an invoice is published, Square follows up based on the invoice configuration. For
    /// example, Square sends the invoice to the customer's email address, charges the customer's
    /// card on file, or does nothing. Square also makes the invoice available on a Square-hosted
    /// invoice page.
    ///
    /// The invoice `status` also changes from `DRAFT` to a status based on the invoice
    /// configuration. For example, the status changes to `UNPAID` if Square emails the invoice or
    /// `PARTIALLY_PAID` if Square charge a card on file for a portion of the invoice amount.
    pub async fn publish_invoice(
        &self,
        invoice_id: &str,
        body: &PublishInvoiceRequest,
    ) -> Result<PublishInvoiceResponse, SquareApiError> {
        let url = format!("{}/{}/publish", &self.url(), invoice_id);
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
