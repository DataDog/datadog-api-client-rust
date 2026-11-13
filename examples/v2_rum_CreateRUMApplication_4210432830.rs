// Create a new RUM application with Product Scales returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::RUMApplicationCreate;
use datadog_api_client::datadogV2::model::RUMApplicationCreateAttributes;
use datadog_api_client::datadogV2::model::RUMApplicationCreateRequest;
use datadog_api_client::datadogV2::model::RUMApplicationCreateType;
use datadog_api_client::datadogV2::model::RUMApplicationPlatformType;
use datadog_api_client::datadogV2::model::RUMEventProcessingState;
use datadog_api_client::datadogV2::model::RUMProductAnalyticsRetentionStateRequest;

#[tokio::main]
async fn main() {
    let body = RUMApplicationCreateRequest::new(RUMApplicationCreate::new(
        RUMApplicationCreateAttributes::new(
            "test-rum-with-product-scales-5c67ebb32077e1d9".to_string(),
        )
        .product_analytics_retention_state(RUMProductAnalyticsRetentionStateRequest::NONE)
        .rum_event_processing_state(RUMEventProcessingState::ERROR_FOCUSED_MODE)
        .type_(RUMApplicationPlatformType::BROWSER),
        RUMApplicationCreateType::RUM_APPLICATION_CREATE,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api.create_rum_application(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
