// Initialize rum segments returns "Default segments created successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_segments::SegmentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.InitializeRumSegments", true);
    let api = SegmentsAPI::with_config(configuration);
    let resp = api.initialize_rum_segments().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
