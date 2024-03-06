// Create a retention filter returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_apm_retention_filters::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        RetentionFilterCreateRequest::new(
            RetentionFilterCreateData::new(
                RetentionFilterCreateAttributes::new(
                    true,
                    SpansFilterCreate::new("@http.status_code:200 service:my-service".to_string()),
                    RetentionFilterType::SPANS_SAMPLING_PROCESSOR,
                    "my retention filter".to_string(),
                    1.0,
                ),
                ApmRetentionFilterType::apm_retention_filter,
            ),
        );
    let configuration = Configuration::new();
    let api = APMRetentionFiltersAPI::with_config(configuration);
    let resp = api.create_apm_retention_filter(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
