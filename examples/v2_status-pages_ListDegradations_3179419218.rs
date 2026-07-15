// List degradations with source ID filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::ListDegradationsOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "degradation" in the system
    let degradation_data_id = uuid::Uuid::parse_str(&std::env::var("DEGRADATION_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .list_degradations(
            ListDegradationsOptionalParams::default().filter_source_id(degradation_data_id.clone()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
