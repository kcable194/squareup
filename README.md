# Square API Client

Square API Client provides a Rust wrapper on top of the Square web APIs.

Forked from https://github.com/cosm-public/rust-square-api-client-lib/tree/main on 1/4/2024.

## Square API

Square APIs enable you to accept payments securely and integrate your app with Square’s first party
product ecosystem. Build full-featured business apps for yourself or millions of Square sellers.

[The Square API Reference](https://developer.squareup.com/reference/square) is organized around core
business workflows: taking payments, managing orders, syncing items and inventory with Square Point
of Sale, creating customer records, managing business locations, and enabling Square sellers to use
your app.

## Usage

### Setting up the client

The client is instantiated most simply with
```rust
use squareup::{config::Configuration, SquareClient};

// This will default to Sandbox, not Production!!
let client = SquareClient::try_new(Configuration::default()).unwrap();
```

For this to work, it's necessary to set an environment variable with the name of `SQUARE_API_TOKEN`
and the value of your API Token string... otherwise, you'll get API errors when making live calls.
You can also add your API Token to the Headers manually when configuring the client. Only do this
for local scripts (if even then).

You also need to set the `SQUARE_ENVIRONMENT` environment variable to either `SANDBOX` or `PRODUCTION`.
If a value is not set, the default is `SANDBOX`.

Other default values include the `2024-06-04` API version, a base URI of `/v2`, a timeout of 60 seconds and no HTTP Client retries.


The standard configuration for production is shown below:
```rust
    // Create square client config
    let config = Configuration {
        environment: Environment::Production, // OPTIONAL if you set the SQUARE_ENVIRONMENT env var
        http_client_config: HttpClientConfiguration::default(),
        base_uri: BaseUri::Default,
    };
```

The Square client can be customized a bit via the properties shown here:
```rust
use squareup::api::{CustomersApi, OrdersApi};
use squareup::config::BaseUri;
use squareup::config::{Configuration, Environment};
use squareup::http::client::HttpClientConfiguration;
use squareup::SquareClient;
use std::time::Duration;
use squareup::http::client::RetryConfiguration;
use squareup::http::Headers;

let mut headers = Headers::default();
headers.set_user_agent("Some User Agent String");
headers.insert("X_SOME_CUSTOM_HEADER", "custom_header_value");
// Not recommended to set auth header, only do this if you are running local scripts
headers.set_authorization("YOUR_API_TOKEN".to_string());

let config = Configuration {
    environment: Environment::Production,
    http_client_config: HttpClientConfiguration {
        timeout: 30,
        user_agent: String::from("Some User Agent String"), // will override what's in headers
        default_headers: headers,
        retry_configuration: RetryConfiguration {
            retries_count: 1,
            min_retry_interval: Duration::from_secs(1),
            max_retry_interval: Duration::from_secs(30 * 60),
            base: 3,
        },
    },
    base_uri: BaseUri::Default,
};

// Create square client
let square_client: SquareClient = SquareClient::try_new(config).unwrap();
```

### Using the client

Once you have a `SquareClient`, inject the client into various APIs. This helps with safety
by not allowing your app access to every API at once.

For example, to access the Customers API and Orders API, you would:
```rust
use squareup::api::{CustomersApi, OrdersApi};
use squareup::config::Configuration;
use squareup::SquareClient;

let square_client: SquareClient = SquareClient::try_new(Configuration::default()).unwrap();
let customers_api: CustomersApi = CustomersApi::new(square_client.clone());
let orders_api: OrdersApi = OrdersApi::new(square_client.clone());
```

## Progress

The intent is to wrap all of the Square APIs in this crate. So far, it includes some of the more
commonly required features.

### Implemented so far

So far, we have the following APIs wrapped in the Rust Square API Client:
- [Apple Pay](https://developer.squareup.com/reference/square/apple-pay-api)
- [Bookings](https://developer.squareup.com/reference/square/bookings-api)
- [Cards](https://developer.squareup.com/reference/square/cards-api)
- [Catalog](https://developer.squareup.com/reference/square/catalog-api)
- [Checkout](https://developer.squareup.com/reference/square/checkout-api)
- [Customer Groups](https://developer.squareup.com/reference/square/customer-groups-api)
- [Customer Segments](https://developer.squareup.com/reference/square/customer-segments-api)
- [Customers](https://developer.squareup.com/reference/square/customers-api)
- [Gift Card Activities](https://developer.squareup.com/reference/square/gift-card-activities-api)
- [Gift Cards](https://developer.squareup.com/reference/square/gift-cards-api)
- [Inventory](https://developer.squareup.com/reference/square/inventory-api)
- [Invoices](https://developer.squareup.com/reference/square/invoices-api)
- [Locations](https://developer.squareup.com/reference/square/locations-api)
- [OAuth](https://developer.squareup.com/reference/square/oauth-api)
- [Orders](https://developer.squareup.com/reference/square/orders-api)
- [Payments](https://developer.squareup.com/reference/square/payments-api)
- [Refunds](https://developer.squareup.com/reference/square/refunds-api)
- [Subscriptions](https://developer.squareup.com/reference/square/subscriptions-api)
- [Team](https://developer.squareup.com/reference/square/team-api)
- [Webhook Subscriptions](https://developer.squareup.com/reference/square/webhook-subscriptions-api)
- [Webhooks](https://developer.squareup.com/reference/square/webhooks)
  - [Bookings](https://developer.squareup.com/reference/square/bookings-api/webhooks)
  - [Cards](https://developer.squareup.com/reference/square/cards-api/webhooks)
  - [Catalog](https://developer.squareup.com/reference/square/catalog-api/webhooks)
  - [Checkout](https://developer.squareup.com/reference/square/checkout-api/webhooks) ("Online Checkout" in Square docs)
  - [Customers](https://developer.squareup.com/reference/square/customers-api/webhooks)
  - [Gift Card Activities](https://developer.squareup.com/reference/square/gift-cards-api/webhooks)
  - [Gift Cards](https://developer.squareup.com/reference/square/gift-card-activities-api)
  - [Inventory](https://developer.squareup.com/reference/square/inventory-api/webhooks)
  - [Invoices](https://developer.squareup.com/reference/square/invoices-api/webhooks)
  - [Locations](https://developer.squareup.com/reference/square/locations-api/webhooks)
  - [OAuth](https://developer.squareup.com/reference/square/o-auth-api/webhooks)
  - [Orders](https://developer.squareup.com/reference/square/orders-api/webhooks)
  - [Payments](https://developer.squareup.com/reference/square/payments-api/webhooks)
  - [Refunds](https://developer.squareup.com/reference/square/refunds-api/webhooks)
  - [Subscriptions](https://developer.squareup.com/reference/square/subscriptions-api/webhooks)
  - [Team](https://developer.squareup.com/reference/square/team-api/webhooks)


### To be implemented

Future versions of this crate will implement the following APIs:
- [Bank Accounts](https://developer.squareup.com/reference/square/bank-accounts-api)
- [Booking Custom Attributes](https://developer.squareup.com/reference/square/booking-custom-attributes-api)
- [Cash Drawers](https://developer.squareup.com/reference/square/cash-drawers-api)
- [Customer Custom Attributes](https://developer.squareup.com/reference/square/customer-custom-attributes-api)
- [Devices](https://developer.squareup.com/reference/square/devices-api)
- [Disputes](https://developer.squareup.com/reference/square/disputes-api)
- [Events](https://developer.squareup.com/reference/square/events-api)
- [Labor](https://developer.squareup.com/reference/square/labor-api)
- [Loyalty](https://developer.squareup.com/reference/square/loyalty-api)
- [Location Custom Attributes](https://developer.squareup.com/reference/square/location-custom-attributes-api)
- [Merchants](https://developer.squareup.com/reference/square/merchants-api)
- [Merchant Custom Attributes](https://developer.squareup.com/reference/square/merchant-custom-attributes-api)
- [Mobile Authorization](https://developer.squareup.com/reference/square/mobile-authorization-api)
- [Order Custom Attributes](https://developer.squareup.com/reference/square/order-custom-attributes-api)
- [Payouts](https://developer.squareup.com/reference/square/payouts-api)
- [Sites](https://developer.squareup.com/reference/square/sites-api)
- [Snippets](https://developer.squareup.com/reference/square/snippets-api)
- [Terminal](https://developer.squareup.com/reference/square/terminal-api)
- [Vendors](https://developer.squareup.com/reference/square/vendors-api)
