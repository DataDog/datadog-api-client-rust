// Create a RUM exclusion filter returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RumExclusionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::RumExclusionFilterCreateData;
use datadog_api_client::datadogV2::model::RumExclusionFilterCreateRequest;
use datadog_api_client::datadogV2::model::RumExclusionFilterEventType;
use datadog_api_client::datadogV2::model::RumExclusionFilterType;

#[tokio::main]
async fn main() {
    let body = RumExclusionFilterCreateRequest::new(RumExclusionFilterCreateData::new(
        RumExclusionFilterCreateAttributes::new(
            "Exclude noisy browser extension errors".to_string(),
        )
        .enabled(true)
        .event_type(RumExclusionFilterEventType::ERROR)
        .query("@error.message:*extension*".to_string()),
        RumExclusionFilterType::EXCLUSION_FILTERS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateExclusionFilter", true);
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .create_exclusion_filter("app_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
