// Get a test result returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::GetSyntheticsTestResultOptionalParams;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_synthetics_test_result(
            "public_id".to_string(),
            "result_id".to_string(),
            GetSyntheticsTestResultOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
