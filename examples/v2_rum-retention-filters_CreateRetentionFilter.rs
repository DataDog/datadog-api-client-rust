// Create a RUM retention filter returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RUMRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RUMRetentionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::RUMRetentionFilterCreateData;
use datadog_api_client::datadogV2::model::RUMRetentionFilterCreateRequest;
use datadog_api_client::datadogV2::model::RUMRetentionFilterEventType;
use datadog_api_client::datadogV2::model::RUMRetentionFilterType;

#[tokio::main]
async fn main() {
    let body = RUMRetentionFilterCreateRequest::new(RUMRetentionFilterCreateData::new(
        RUMRetentionFilterCreateAttributes::new(
            RUMRetentionFilterEventType::SESSION,
            "Test creating retention filter".to_string(),
            50.0,
        )
        .enabled(true)
        .query("custom_query".to_string()),
        RUMRetentionFilterType::RETENTION_FILTERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .create_retention_filter("a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
