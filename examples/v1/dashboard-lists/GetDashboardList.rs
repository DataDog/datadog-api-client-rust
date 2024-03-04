// Get a dashboard list returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboard_lists::DashboardListsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "dashboard_list" in the system
    let dashboard_list_id: i64 = std::env::var("DASHBOARD_LIST_ID").unwrap().parse().unwrap();
    let configuration = Configuration::new();
    let api = DashboardListsAPI::with_config(configuration);
    let resp = api.get_dashboard_list().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
