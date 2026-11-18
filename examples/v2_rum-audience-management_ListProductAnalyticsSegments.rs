// List segments returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::ListProductAnalyticsSegmentsOptionalParams;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_product_analytics_segments", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api
        .list_product_analytics_segments(ListProductAnalyticsSegmentsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
