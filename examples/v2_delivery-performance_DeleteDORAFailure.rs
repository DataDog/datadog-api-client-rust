// Delete an incident event returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_delivery_performance::DeliveryPerformanceAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DeliveryPerformanceAPI::with_config(configuration);
    let resp = api.delete_dora_failure("failure_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
