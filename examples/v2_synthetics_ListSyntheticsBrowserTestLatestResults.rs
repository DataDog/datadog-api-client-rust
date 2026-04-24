// Get a browser test's latest results returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::ListSyntheticsBrowserTestLatestResultsOptionalParams;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .list_synthetics_browser_test_latest_results(
            "public_id".to_string(),
            ListSyntheticsBrowserTestLatestResultsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
