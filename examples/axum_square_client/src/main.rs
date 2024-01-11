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
use tokio::net::TcpListener;

// struct that holds shared app state that axum will inject
struct AppState {
    customers_api: CustomersApi,
    orders_api: OrdersApi,
}

impl AppState {
    // convenience method for wrapping app state in an Arc
    pub fn new(customers_api: CustomersApi, orders_api: OrdersApi) -> Arc<AppState> {
        let app_state = AppState {
            customers_api,
            orders_api,
        };

        Arc::new(app_state)
    }
}

#[tokio::main]
async fn main() {
    // Create square client config
    let config = Configuration {
        environment: Environment::Production,
        square_version: SquareVersion::SquareVersion,
        http_client_config: HttpClientConfiguration::default(),
        access_token: "".to_string(),
        base_uri: String::from("/v2"),
    };

    // Create square client, and instantiate any api structs you want
    let square_client: SquareClient = SquareClient::try_new(config).unwrap();
    let customers_api: CustomersApi = CustomersApi::new(square_client.to_owned());
    let orders_api: OrdersApi = OrdersApi::new(square_client.to_owned());

    // Instantiate app state struct, wrapped in an Arc for sharing across threads
    let app_state: Arc<AppState> = AppState::new(customers_api, orders_api);

    // Add some routes to the application, as well as our app state
    let app: Router = Router::new()
        .route("/customers/:num", get(customers))
        .route("/orders/:location_id/:num", get(orders))
        .with_state(app_state);

    // run app with hyper
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn customers(
    Path(num): Path<usize>,
    State(app_state): State<Arc<AppState>>,
) -> Json<Customer> {
    // Begin getting customer information
    let list_customer_params = ListCustomersParameters {
        cursor: "".to_string(),
        limit: Some(100),
        sort_field: SortCustomersField::Default,
        sort_order: SortOrder::Asc,
        count: Some(true),
    };

    // call customers api and get list of customers
    let customers_response: ListCustomersResponse = app_state
        .customers_api
        .list_customers(&list_customer_params)
        .await
        .unwrap();

    // unwrap response and send back as serialized json
    let customers = customers_response.customers.unwrap();
    let customer: &Customer = customers.get(num).unwrap();

    Json(customer.to_owned())
}

async fn orders(Path((location_id, num)): Path<(String, usize)>, State(app_state): State<Arc<AppState>>) -> Json<Order> {
    // Create search orders request object
    let search_request = SearchOrdersRequest {
        location_ids: Some(vec![location_id]),
        cursor: None,
        query: None,
        limit: Some(100),
        return_entries: Some(false),
    };

    // call orders api to get list of orders
    let search_response = app_state
        .orders_api
        .search_orders(&search_request)
        .await
        .unwrap();

    // unwrap response and send back as serialized json
    let orders = search_response.orders.unwrap();
    let order: &Order = orders.get(num).unwrap();

    Json(order.to_owned())
}
