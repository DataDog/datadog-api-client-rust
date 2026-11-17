// List Product Analytics launches returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ListProductAnalyticsLaunchesOptionalParams;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_product_analytics_launches", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api
        .list_product_analytics_launches(ListProductAnalyticsLaunchesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
