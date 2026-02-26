// Get maintenance returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::GetMaintenanceOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = uuid::Uuid::parse_str(&std::env::var("STATUS_PAGE_DATA_ID").unwrap())
        .expect("Invalid UUID");

    // there is a valid "maintenance" in the system
    let maintenance_data_id = uuid::Uuid::parse_str(&std::env::var("MAINTENANCE_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .get_maintenance(
            status_page_data_id.clone(),
            maintenance_data_id.clone(),
            GetMaintenanceOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
