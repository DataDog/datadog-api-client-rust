// Delete a retention filter returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_apm_retention_filters::*;

#[tokio::main]
async fn main() {
    // there is a valid "retention_filter" in the system
    let retention_filter_data_id = std::env::var("RETENTION_FILTER_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = APMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .delete_apm_retention_filter(retention_filter_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
