// Search Synthetic tests returns "OK - Returns the list of Synthetic tests
// matching the search." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SearchTestsOptionalParams;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.search_tests(SearchTestsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
