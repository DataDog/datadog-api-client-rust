// Delete a security filter returns "No Content" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;

#[tokio::main]
async fn main() {
    // there is a valid "security_filter" in the system
    let security_filter_data_id = std::env::var("SECURITY_FILTER_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .delete_security_filter(security_filter_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}