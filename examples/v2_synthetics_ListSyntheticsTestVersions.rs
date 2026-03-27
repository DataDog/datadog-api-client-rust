// Get version history of a test returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::ListSyntheticsTestVersionsOptionalParams;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .list_synthetics_test_versions(
            "public_id".to_string(),
            ListSyntheticsTestVersionsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
