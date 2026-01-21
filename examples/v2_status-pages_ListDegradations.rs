// List degradations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::ListDegradationsOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .list_degradations(ListDegradationsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
