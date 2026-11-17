// Update a Product Analytics launch returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchPatchAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchPatchData;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchPatchRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchPhase;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchType;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsLaunchPatchRequest::new(ProductAnalyticsLaunchPatchData::new(
        ProductAnalyticsLaunchPatchAttributes::new().phase(ProductAnalyticsLaunchPhase::SETUP),
        "5e3c7a1b-2d4f-4e6a-8b9c-0d1e2f3a4b5c".to_string(),
        ProductAnalyticsLaunchType::LIFTOFF,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_product_analytics_launch", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api
        .update_product_analytics_launch("5e3c7a1b-2d4f-4e6a-8b9c-0d1e2f3a4b5c".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
