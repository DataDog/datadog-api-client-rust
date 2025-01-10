// Add custom screenboard dashboard to an existing dashboard list returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboard_lists::DashboardListsAPI;
use datadog_api_client::datadogV2::model::DashboardListAddItemsRequest;
use datadog_api_client::datadogV2::model::DashboardListItemRequest;
use datadog_api_client::datadogV2::model::DashboardType;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard_list" in the system
    let dashboard_list_id: i64 = std::env::var("DASHBOARD_LIST_ID").unwrap().parse().unwrap();
    // there is a valid "screenboard_dashboard" in the system
    let screenboard_dashboard_id = std::env::var("SCREENBOARD_DASHBOARD_ID").unwrap();
    let body = DashboardListAddItemsRequest::new().dashboards(vec![DashboardListItemRequest::new(
        screenboard_dashboard_id.clone(),
        DashboardType::CUSTOM_SCREENBOARD,
    )]);

    let configuration = datadog::Configuration::new();
    let api = DashboardListsAPI::with_config(configuration);
    let resp = api
        .create_dashboard_list_items(dashboard_list_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
