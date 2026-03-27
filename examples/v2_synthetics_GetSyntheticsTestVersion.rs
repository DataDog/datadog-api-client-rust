// Get a specific version of a test returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::GetSyntheticsTestVersionOptionalParams;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_synthetics_test_version(
            "public_id".to_string(),
            9223372036854775807,
            GetSyntheticsTestVersionOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
