// Remove a test from a Synthetics downtime returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .remove_test_from_synthetics_downtime(
            "00000000-0000-0000-0000-000000000001".to_string(),
            "abc-def-123".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
