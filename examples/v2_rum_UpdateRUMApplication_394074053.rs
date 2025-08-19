// Update a RUM application with Product Scales returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::RUMApplicationUpdate;
use datadog_api_client::datadogV2::model::RUMApplicationUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMApplicationUpdateRequest;
use datadog_api_client::datadogV2::model::RUMApplicationUpdateType;
use datadog_api_client::datadogV2::model::RUMEventProcessingState;
use datadog_api_client::datadogV2::model::RUMProductAnalyticsRetentionState;

#[tokio::main]
async fn main() {
    // there is a valid "rum_application" in the system
    let rum_application_data_id = std::env::var("RUM_APPLICATION_DATA_ID").unwrap();
    let body = RUMApplicationUpdateRequest::new(
        RUMApplicationUpdate::new(
            rum_application_data_id.clone(),
            RUMApplicationUpdateType::RUM_APPLICATION_UPDATE,
        )
        .attributes(
            RUMApplicationUpdateAttributes::new()
                .name("updated_rum_with_product_scales".to_string())
                .product_analytics_retention_state(RUMProductAnalyticsRetentionState::MAX)
                .rum_event_processing_state(RUMEventProcessingState::ALL),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api
        .update_rum_application(rum_application_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
