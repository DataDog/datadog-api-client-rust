// Get a permanent RUM retention filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RUMRetentionFiltersAPI;
use datadog_api_client::datadogV2::model::RUMPermanentRetentionFilterID;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .get_permanent_retention_filter(
            "app_id".to_string(),
            RUMPermanentRetentionFilterID::SYNTHETICS_SESSIONS,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
