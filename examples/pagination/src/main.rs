use squareup::api::CustomersApi;
use squareup::config::{Configuration, Environment, SquareVersion};
use squareup::http::client::HttpClientConfiguration;
use squareup::models::enums::{SortCustomersField, SortOrder};
use squareup::models::{ListCustomersParameters, ListCustomersResponse};
use squareup::SquareClient;

#[tokio::main]
async fn main() {
    // Create client config, square client, and customers_api
    let config = Configuration {
        environment: Environment::Production,
        square_version: SquareVersion::default(),
        http_client_config: HttpClientConfiguration::default(),
        access_token: "".to_string(),
        base_uri: String::from("/v2"),
    };

    let square_client: SquareClient = SquareClient::try_new(config).unwrap();
    let customers_api: CustomersApi = CustomersApi::new(square_client.to_owned());

    // create vector to store customer id's
    let mut customer_ids: Vec<String> = Vec::new();

    // Define list customers parameters
    let mut list_customer_params = ListCustomersParameters {
        // Start with empty cursor
        cursor: "".to_string(),
        limit: Some(100),
        sort_field: SortCustomersField::Default,
        sort_order: SortOrder::Asc,
        count: Some(true),
    };

    // Call api for first time. This will return a cursor, if necessary
    let customers_response: ListCustomersResponse = customers_api
        .list_customers(&list_customer_params)
        .await
        .unwrap();

    // Since we set count=true, we can get the total number of customers in Square's data
    let number_of_customers = customers_response.count.unwrap();

    // Add ids to vector from first response
    for customer in customers_response.customers.unwrap() {
        customer_ids.push(customer.id.unwrap());
    }

    // User cursor to continue to query data from the api, while Some(cursor) exists
    let mut cursor = customers_response.cursor;
    while let Some(c) = cursor {
        // update cursor in list customer params struct
        list_customer_params.cursor = c;

        // call the api with the new cursor
        let customers_response: ListCustomersResponse = customers_api
            .list_customers(&list_customer_params)
            .await
            .unwrap();

        // Add more ids to vector
        for customer in customers_response.customers.unwrap() {
            customer_ids.push(customer.id.unwrap());
        }

        // update cursor for next iteration
        cursor = customers_response.cursor;
    }

    // Show that all customer ids were added to our vector
    assert_eq!(number_of_customers, customer_ids.len() as i64);
}
