// Update a RUM exclusion filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RUMRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RUMExclusionFilterEventType;
use datadog_api_client::datadogV2::model::RUMExclusionFilterType;
use datadog_api_client::datadogV2::model::RUMExclusionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMExclusionFilterUpdateData;
use datadog_api_client::datadogV2::model::RUMExclusionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RUMExclusionFilterUpdateRequest::new(RUMExclusionFilterUpdateData::new(
        RUMExclusionFilterUpdateAttributes::new()
            .enabled(true)
            .event_type(RUMExclusionFilterEventType::ERROR)
            .name("Exclude noisy browser extension errors".to_string())
            .query("@error.message:*extension*".to_string()),
        "051601eb-54a0-abc0-03f9-cc02efa18892".to_string(),
        RUMExclusionFilterType::EXCLUSION_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .update_exclusion_filter("app_id".to_string(), "ef_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
