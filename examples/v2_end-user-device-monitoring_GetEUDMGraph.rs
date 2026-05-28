// Get device counts grouped by attribute returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_end_user_device_monitoring::EndUserDeviceMonitoringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = EndUserDeviceMonitoringAPI::with_config(configuration);
    let resp = api.get_eudm_graph("by".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
