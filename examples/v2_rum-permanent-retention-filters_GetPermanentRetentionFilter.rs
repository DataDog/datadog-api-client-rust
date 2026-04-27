// Get a permanent retention filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_permanent_retention_filters::RumPermanentRetentionFiltersAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumPermanentRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .get_permanent_retention_filter(
            "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(),
            "default_synthetics".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
