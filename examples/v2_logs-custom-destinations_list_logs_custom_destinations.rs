// Get all custom destinations returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_custom_destinations::LogsCustomDestinationsAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsCustomDestinationsAPI::with_config(configuration);
    let resp = api.list_logs_custom_destinations().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
