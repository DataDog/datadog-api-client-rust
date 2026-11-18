// Unlink a Product Analytics monitor from its chart returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUnlinkAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUnlinkData;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUnlinkRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUnlinkRequestType;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsMonitorUnlinkRequest::new(ProductAnalyticsMonitorUnlinkData::new(
        ProductAnalyticsMonitorUnlinkAttributes::new("chart-1".to_string())
            .dashboard_id("dash-1".to_string()),
        ProductAnalyticsMonitorUnlinkRequestType::PA_MONITOR_UNLINK_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.remove_chart_from_product_analytics_monitor", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api
        .remove_chart_from_product_analytics_monitor("12345".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
