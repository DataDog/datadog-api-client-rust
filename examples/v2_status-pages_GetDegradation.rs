// Get degradation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::GetDegradationOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = std::env::var("STATUS_PAGE_DATA_ID").unwrap();

    // there is a valid "degradation" in the system
    let degradation_data_id = std::env::var("DEGRADATION_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .get_degradation(
            status_page_data_id.clone(),
            degradation_data_id.clone(),
            GetDegradationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
