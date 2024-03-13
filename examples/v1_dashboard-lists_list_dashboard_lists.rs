// Get all dashboard lists returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboard_lists::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = DashboardListsAPI::with_config(configuration);
    let resp = api.list_dashboard_lists().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}