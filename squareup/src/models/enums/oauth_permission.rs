//! Model for OAuthPermission enum

use serde::{Deserialize, Serialize};

/// Permissions that the application is requesting.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OAuthPermission {
    /// HTTP Method: GET
    ///
    /// Grants read access to bank account information associated with the targeted Square account.
    /// For example, to call the Connect v1 ListBankAccounts endpoint.
    BankAccountsRead,
    /// HTTP Method: GET
    ///
    /// Grants read access to cash drawer shift information. For example, to call the
    /// ListCashDrawerShifts endpoint.
    CashDrawerRead,
    /// HTTP Method: GET
    ///
    /// Grants read access to customer information. For example, to call the ListCustomers endpoint.
    CustomersRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to customer information. For example, to create and update customer
    /// profiles.
    CustomersWrite,
    /// HTTP Method: POST, GET
    ///
    /// Grants read/write access to device credentials information. For example, to call the
    /// CreateDeviceCode endpoint.
    DeviceCredentialManagement,
    /// HTTP Method: GET
    ///
    /// Grants read access to bank account information associated with the targeted Square account.
    /// For example, to call the Connect v1 ListBankAccounts endpoint.
    EmployeesRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to employee profile information. For example, to create and modify
    /// employee profiles.
    EmployeesWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to inventory information. For example, to call the RetrieveInventoryCount
    /// endpoint.
    InventoryRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to inventory information. For example, to call the BatchChangeInventory
    /// endpoint.
    InventoryWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to product catalog information. For example, to obtain objects in a
    /// product catalog.
    ItemsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to product catalog information. For example, to modify or add to a
    /// product catalog.
    ItemsWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to loyalty information. For example, to call the ListLoyaltyPrograms
    /// endpoint.
    LoyaltyRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to loyalty information. For example, to call the CreateLoyaltyAccount
    /// endpoint.
    LoyaltyWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to business and location information. For example, to obtain a location
    /// ID for subsequent activity.
    MerchantProfileRead,
    /// HTTP Method: GET
    ///
    /// Grants read access to order information. For example, to call the BatchRetrieveOrders
    /// endpoint.
    OrdersRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to order information. For example, to call the CreateCheckout endpoint.
    OrdersWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to transaction and refund information. For example, to call the
    /// RetrieveTransaction endpoint.
    PaymentsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to transaction and refunds information. For example, to process
    /// payments with the Payments or Checkout API.
    PaymentsWrite,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Allow third party applications to deduct a portion of each transaction amount. Required to
    /// use multiparty transaction functionality with the Payments API.
    PaymentsWriteAdditionalRecipients,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to payments and refunds information. For example, to process in-person
    /// payments.
    PaymentsWriteInPerson,
    /// HTTP Method: GET
    ///
    /// Grants read access to settlement (deposit) information. For example, to call the Connect
    /// v1 ListSettlements endpoint.
    SettlementsRead,
    /// HTTP Method: GET
    ///
    /// Grants read access to employee timecard information. For example, to call the Connect v2
    /// SearchShifts endpoint.
    TimecardsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to employee shift information. For example, to create and modify
    /// employee shifts.
    TimecardsWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to employee timecard settings information. For example, to call the
    /// GetBreakType endpoint.
    TimecardsSettingsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to employee timecard settings information. For example, to call the
    /// UpdateBreakType endpoint.
    TimecardsSettingsWrite,
    /// HTTP Method: GET, POST
    ///
    /// Grants read access to booking information. For example, to call the RetrieveBooking endpoint.
    AppointmentsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to employee profile information. For example, to create and modify
    /// employee profiles.
    AppointmentsWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to booking business settings. For example, to call the
    /// ListTeamMemberBookingProfiles endpoint.
    AppointmentsBusinessSettingsRead,
    /// HTTP Method: GET, POST
    ///
    /// Grants read access to invoice information. For example, to call the ListInvoices endpoint.
    InvoicesRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to invoice information. For example, to call the CreateInvoice endpoint.
    InvoicesWrite,
    /// HTTP Method: GET, POST
    ///
    /// Grants read access to subscription information. For example, to call the
    /// RetrieveSubscription endpoint.
    SubscriptionsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to subscription information. For example, to call the CreateSubscription
    /// endpoint.
    SubscriptionsWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to dispute information. For example, to call the RetrieveDispute endpoint.
    DisputesRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to dispute information. For example, to call the SubmitEvidence endpoint.
    DisputesWrite,
    /// HTTP Method: GET, POST
    ///
    /// Grants read access to gift card information. For example, to call the RetrieveGiftCard
    /// endpoint.
    GiftcardsRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to gift card information. For example, to call the CreateGiftCard
    /// endpoint.
    GiftcardsWrite,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Write access to ECOM online store snippets on published websites.
    OnlineStoreSnippetsWrite,
    /// HTTP Method: GET, POST
    ///
    /// Read access to ECOM online store snippets on published websites.
    OnlineStoreSnippetsRead,
    /// HTTP Method: GET, POST
    ///
    /// Read access to ECOM online store site details.
    OnlineStoreSiteRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Allows the developer to process payments on behalf of a seller using a shared on file
    /// payment method.
    PaymentsWriteSharedOnfile,
    /// HTTP Method: GET, POST
    ///
    /// Grants read access to all of a seller's booking information, calendar, and business details.
    /// This permission must be accompanied by the APPOINTMENTS_READ permission.
    AppointmentsAllRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to all booking details, including double-booking a seller. This
    /// permission must be accompanied by the APPOINTMENTS_WRITE permission.
    AppointmentsAllWrite,
    /// HTTP Method: POST, PUT
    ///
    /// Grants write access to business and location information. For example, to create a new
    /// location or update the business hours at an existing location.
    MerchantProfileWrite,
    /// HTTP Method: GET, POST
    ///
    /// Grants read access to vendor information, for example, when calling the RetrieveVendor
    /// endpoint.
    VendorRead,
    /// HTTP Method: POST, PUT, DELETE
    ///
    /// Grants write access to vendor information, for example, when calling the BulkUpdateVendors
    /// endpoint.
    VendorWrite,
    /// HTTP Method: GET
    ///
    /// Grants read access to payouts and payout entries information. For example, to call the
    /// Connect v2 ListPayouts endpoint.
    PayoutsRead,
    /// HTTP Method: GET
    ///
    /// Grants read access to device information. For example, to call the GetDevice and
    /// ListDevices endpoints.
    DevicesRead,
}
