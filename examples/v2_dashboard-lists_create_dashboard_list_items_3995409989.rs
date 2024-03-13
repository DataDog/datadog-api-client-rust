// Add custom timeboard dashboard to an existing dashboard list returns "OK"
// response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_dashboard_lists::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard_list" in the system
    let dashboard_list_id: i64 = std::env::var("DASHBOARD_LIST_ID").unwrap().parse().unwrap();

    // there is a valid "dashboard" in the system
    let dashboard_id = std::env::var("DASHBOARD_ID").unwrap();
    let body = DashboardListAddItemsRequest::new().dashboards(vec![DashboardListItemRequest::new(
        dashboard_id.clone(),
        DashboardType::CUSTOM_TIMEBOARD,
    )]);
    let configuration = Configuration::new();
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
