// Delete items from a dashboard list returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_dashboard_lists::DashboardListsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        DashboardListDeleteItemsRequest
        ::new().dashboards(
            vec![DashboardListItemRequest::new("q5j-nti-fv6".to_string(), DashboardType::HOST_TIMEBOARD)],
        );
    let configuration = Configuration::new();
    let api = DashboardListsAPI::with_config(configuration);
    let resp = api.delete_dashboard_list_items(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
