// Get all suppression rules returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.list_security_monitoring_suppressions().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
