// Create a RUM exclusion filter returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RUMRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RUMExclusionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::RUMExclusionFilterCreateData;
use datadog_api_client::datadogV2::model::RUMExclusionFilterCreateRequest;
use datadog_api_client::datadogV2::model::RUMExclusionFilterEventType;
use datadog_api_client::datadogV2::model::RUMExclusionFilterType;

#[tokio::main]
async fn main() {
    let body = RUMExclusionFilterCreateRequest::new(RUMExclusionFilterCreateData::new(
        RUMExclusionFilterCreateAttributes::new(
            "Exclude noisy browser extension errors".to_string(),
        )
        .enabled(true)
        .event_type(RUMExclusionFilterEventType::ERROR)
        .query("@error.message:*extension*".to_string()),
        RUMExclusionFilterType::EXCLUSION_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .create_exclusion_filter("app_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
