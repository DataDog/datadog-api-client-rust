// Update a Product Analytics monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorOptions;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorThresholds;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUpdateAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUpdateData;
use datadog_api_client::datadogV2::model::ProductAnalyticsMonitorUpdateRequest;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsMonitorUpdateRequest::new(ProductAnalyticsMonitorUpdateData::new(
        ProductAnalyticsMonitorUpdateAttributes::new()
            .name("Funnel conversion drop".to_string())
            .options(
                ProductAnalyticsMonitorOptions::new().thresholds(
                    ProductAnalyticsMonitorThresholds::new()
                        .critical(1.0 as f64)
                        .warning(1.5 as f64),
                ),
            ),
        ProductAnalyticsMonitorRequestType::PA_MONITOR_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_product_analytics_monitor", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api
        .update_product_analytics_monitor("12345".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
