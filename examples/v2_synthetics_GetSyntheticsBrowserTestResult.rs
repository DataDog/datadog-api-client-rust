// Get a browser test result returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::GetSyntheticsBrowserTestResultOptionalParams;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_synthetics_browser_test_result(
            "public_id".to_string(),
            "result_id".to_string(),
            GetSyntheticsBrowserTestResultOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
