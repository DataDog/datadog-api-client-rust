// List RUM segment templates returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_user_segments::RUMUserSegmentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListRumSegmentTemplates", true);
    let api = RUMUserSegmentsAPI::with_config(configuration);
    let resp = api.list_rum_segment_templates().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
