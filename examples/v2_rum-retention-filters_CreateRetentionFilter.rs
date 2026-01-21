// Create a RUM retention filter returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RumRetentionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::RumRetentionFilterCreateData;
use datadog_api_client::datadogV2::model::RumRetentionFilterCreateRequest;
use datadog_api_client::datadogV2::model::RumRetentionFilterEventType;
use datadog_api_client::datadogV2::model::RumRetentionFilterType;

#[tokio::main]
async fn main() {
    let body = RumRetentionFilterCreateRequest::new(RumRetentionFilterCreateData::new(
        RumRetentionFilterCreateAttributes::new(
            RumRetentionFilterEventType::SESSION,
            "Test creating retention filter".to_string(),
            50.0,
        )
        .enabled(true)
        .query("custom_query".to_string()),
        RumRetentionFilterType::RETENTION_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .create_retention_filter("a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
