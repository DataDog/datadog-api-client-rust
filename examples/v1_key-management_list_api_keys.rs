// Get all API keys returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_key_management::KeyManagementAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.list_api_keys().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
