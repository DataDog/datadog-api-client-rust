// Update a RUM exclusion filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RumExclusionFilterEventType;
use datadog_api_client::datadogV2::model::RumExclusionFilterType;
use datadog_api_client::datadogV2::model::RumExclusionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RumExclusionFilterUpdateData;
use datadog_api_client::datadogV2::model::RumExclusionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RumExclusionFilterUpdateRequest::new(RumExclusionFilterUpdateData::new(
        RumExclusionFilterUpdateAttributes::new()
            .enabled(true)
            .event_type(RumExclusionFilterEventType::ERROR)
            .name("Exclude noisy browser extension errors".to_string())
            .query("@error.message:*extension*".to_string()),
        "051601eb-54a0-abc0-03f9-cc02efa18892".to_string(),
        RumExclusionFilterType::EXCLUSION_FILTERS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateExclusionFilter", true);
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .update_exclusion_filter("app_id".to_string(), "ef_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
