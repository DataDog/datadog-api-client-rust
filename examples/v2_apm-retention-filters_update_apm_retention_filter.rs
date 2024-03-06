// Update a retention filter returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_apm_retention_filters::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "retention_filter" in the system
    let retention_filter_data_id = std::env::var("RETENTION_FILTER_DATA_ID").unwrap();
    let body =
        RetentionFilterUpdateRequest::new(
            RetentionFilterUpdateData::new(
                RetentionFilterCreateAttributes::new(
                    true,
                    SpansFilterCreate::new("@_top_level:1 test:service-demo".to_string()),
                    RetentionFilterType::SPANS_SAMPLING_PROCESSOR,
                    "test".to_string(),
                    0.9,
                ),
                "test-id".to_string(),
                ApmRetentionFilterType::apm_retention_filter,
            ),
        );
    let configuration = Configuration::new();
    let api = APMRetentionFiltersAPI::with_config(configuration);
    let resp = api.update_apm_retention_filter(retention_filter_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
