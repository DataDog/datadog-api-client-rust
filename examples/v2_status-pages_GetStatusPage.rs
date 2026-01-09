// Get status page returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::GetStatusPageOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = std::env::var("STATUS_PAGE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .get_status_page(
            status_page_data_id.clone(),
            GetStatusPageOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
