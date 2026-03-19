// Get a RUM segment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_user_segments::RUMUserSegmentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumSegment", true);
    let api = RUMUserSegmentsAPI::with_config(configuration);
    let resp = api
        .get_rum_segment("a1b2c3d4-1234-5678-9abc-123456789abc".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
