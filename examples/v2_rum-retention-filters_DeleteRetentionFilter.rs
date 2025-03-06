// Delete a RUM retention filter returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .delete_retention_filter(
            "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(),
            "fe34ee09-14cf-4976-9362-08044c0dea80".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
