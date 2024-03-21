// Get the list of all Synthetic tests returns "OK - Returns the list of all
// Synthetic tests." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::ListTestsOptionalParams;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.list_tests(ListTestsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
