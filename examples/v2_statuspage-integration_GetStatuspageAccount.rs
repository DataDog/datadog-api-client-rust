// Get the Statuspage account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_statuspage_integration::StatuspageIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StatuspageIntegrationAPI::with_config(configuration);
    let resp = api.get_statuspage_account().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
