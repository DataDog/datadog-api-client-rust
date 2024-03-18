// Get deleted dashboards returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api
        .list_dashboards(ListDashboardsOptionalParams::default().filter_deleted(true))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
