// Get a hardcoded retention filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters_hardcoded::RUMRetentionFiltersHardcodedAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersHardcodedAPI::with_config(configuration);
    let resp = api
        .get_hardcoded_retention_filter(
            "Example-RUM-Retention-Filters-Hardcoded".to_string(),
            "Example-RUM-Retention-Filters-Hardcoded".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
