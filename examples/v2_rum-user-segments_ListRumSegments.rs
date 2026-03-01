// List all RUM segments returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_user_segments::ListRumSegmentsOptionalParams;
use datadog_api_client::datadogV2::api_rum_user_segments::RUMUserSegmentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListRumSegments", true);
    let api = RUMUserSegmentsAPI::with_config(configuration);
    let resp = api
        .list_rum_segments(ListRumSegmentsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
