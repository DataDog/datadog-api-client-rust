// Re-order retention filters returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_apm_retention_filters::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = ReorderRetentionFiltersRequest::new(vec![
        RetentionFilterWithoutAttributes::new(
            "jdZrilSJQLqzb6Cu7aub9Q".to_string(),
            ApmRetentionFilterType::apm_retention_filter,
        ),
        RetentionFilterWithoutAttributes::new(
            "7RBOb7dLSYWI01yc3pIH8w".to_string(),
            ApmRetentionFilterType::apm_retention_filter,
        ),
    ]);
    let configuration = Configuration::new();
    let api = APMRetentionFiltersAPI::with_config(configuration);
    let resp = api.reorder_apm_retention_filters(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
