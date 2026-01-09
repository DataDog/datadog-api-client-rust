// Get component returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::GetComponentOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_attributes_components_0_id =
        std::env::var("STATUS_PAGE_DATA_ATTRIBUTES_COMPONENTS_0_ID").unwrap();
    let status_page_data_id = std::env::var("STATUS_PAGE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .get_component(
            status_page_data_id.clone(),
            status_page_data_attributes_components_0_id.clone(),
            GetComponentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
