// Get all RUM retention filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .list_retention_filters("1d4b9c34-7ac4-423a-91cf-9902d926e9b3".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
