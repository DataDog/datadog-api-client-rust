// Update items of a dashboard list returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_dashboard_lists::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard_list" in the system
    let dashboard_list_id: i64 = std::env::var("DASHBOARD_LIST_ID").unwrap().parse().unwrap();

    // there is a valid "screenboard_dashboard" in the system
    let screenboard_dashboard_id = std::env::var("SCREENBOARD_DASHBOARD_ID").unwrap();
    let body =
        DashboardListUpdateItemsRequest::new().dashboards(vec![DashboardListItemRequest::new(
            screenboard_dashboard_id.clone(),
            DashboardType::CUSTOM_SCREENBOARD,
        )]);
    let configuration = Configuration::new();
    let api = DashboardListsAPI::with_config(configuration);
    let resp = api
        .update_dashboard_list_items(dashboard_list_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
