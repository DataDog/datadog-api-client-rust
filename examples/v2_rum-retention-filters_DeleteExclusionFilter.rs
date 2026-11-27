// Delete a RUM exclusion filter returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_filters::RumRetentionFiltersAPI;

#[tokio::main]
async fn main() {
    // there is a valid "rum_exclusion_filter" in the system
    let rum_exclusion_filter_data_id = std::env::var("RUM_EXCLUSION_FILTER_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RumRetentionFiltersAPI::with_config(configuration);
    let resp = api
        .delete_exclusion_filter(
            "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(),
            rum_exclusion_filter_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
