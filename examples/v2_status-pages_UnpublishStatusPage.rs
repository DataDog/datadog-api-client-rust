// Unpublish status page returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .unpublish_status_page(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
