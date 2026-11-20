// Update a RUM retention filter returns "Updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RUMRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RUMRetentionFilterEventType;
use datadog_api_client::datadogV2::model::RUMRetentionFilterType;
use datadog_api_client::datadogV2::model::RUMRetentionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMRetentionFilterUpdateData;
use datadog_api_client::datadogV2::model::RUMRetentionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RUMRetentionFilterUpdateRequest::new(RUMRetentionFilterUpdateData::new(
        RUMRetentionFilterUpdateAttributes::new()
            .enabled(true)
            .event_type(RUMRetentionFilterEventType::VIEW)
            .name("Test updating retention filter".to_string())
            .query("view_query".to_string())
            .sample_rate(100.0 as f64),
        "4b95d361-f65d-4515-9824-c9aaeba5ac2a".to_string(),
        RUMRetentionFilterType::RETENTION_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .update_retention_filter(
            "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(),
            "4b95d361-f65d-4515-9824-c9aaeba5ac2a".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
