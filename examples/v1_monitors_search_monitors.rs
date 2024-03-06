// Monitors search returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_monitors::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .search_monitors(SearchMonitorsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}