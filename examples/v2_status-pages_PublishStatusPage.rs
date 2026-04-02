// Publish status page returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "unpublished_status_page" in the system
    let unpublished_status_page_data_id =
        uuid::Uuid::parse_str(&std::env::var("UNPUBLISHED_STATUS_PAGE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .publish_status_page(unpublished_status_page_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
