use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use squareup::api::{CustomersApi, OrdersApi};
use squareup::config::SquareVersion;
use squareup::config::{Configuration, Environment};
use squareup::http::client::HttpClientConfiguration;
use squareup::models::enums::{SortCustomersField, SortOrder};
use squareup::models::{
    Customer, ListCustomersParameters, ListCustomersResponse, Order, SearchOrdersRequest,
};
use squareup::SquareClient;
use std::sync::Arc;

struct AppState {
    customers_api: CustomersApi,
    orders_api: OrdersApi,
}

#[tokio::main]
async fn main() {
    let config = Configuration {
        environment: Environment::Production,
        square_version: SquareVersion::default(),
        http_client_config: HttpClientConfiguration::default(),
        access_token: "".to_string(),
        base_uri: String::from("/v2"),
    };

    let square_client: SquareClient = SquareClient::try_new(config).unwrap();
    let customers_api: CustomersApi = CustomersApi::new(square_client.clone());
    let orders_api: OrdersApi = OrdersApi::new(square_client.clone());

    let app_state = AppState {
        customers_api,
        orders_api,
    };
    let app_state_arc = Arc::new(app_state);

    // build our application with some routes
    let app = Router::new()
        .route("/customers/:num", get(customers))
        .route("/orders/:location_id/:num", get(orders))
        .with_state(app_state_arc);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn customers(
    Path(num): Path<usize>,
    State(app_state): State<Arc<AppState>>,
) -> Json<Customer> {
    let state = app_state.clone();

    // Begin getting customer information
    let list_customer_params = ListCustomersParameters {
        cursor: "".to_string(),
        limit: Some(100),
        sort_field: SortCustomersField::Default,
        sort_order: SortOrder::Asc,
        count: Some(true),
    };

    // call customers api and get list of customers
    let customers_response: ListCustomersResponse = state
        .customers_api
        .list_customers(&list_customer_params)
        .await
        .unwrap();

    let customers = customers_response.customers.unwrap();
    let customer: &Customer = customers.get(num).unwrap();
    Json(customer.clone())
}

async fn orders(Path((location_id, num)): Path<(String, usize)>, State(app_state): State<Arc<AppState>>) -> Json<Order> {
    let state = app_state.clone();

    let search_request = SearchOrdersRequest {
        location_ids: Some(vec![location_id]),
        cursor: None,
        query: None,
        limit: Some(100),
        return_entries: Some(false),
    };

    let search_response = state
        .orders_api
        .search_orders(&search_request)
        .await
        .unwrap();

    let orders = search_response.orders.unwrap();
    let order: &Order = orders.get(num).unwrap();
    Json(order.clone())
}
