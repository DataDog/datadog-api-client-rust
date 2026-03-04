// List maintenances returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::ListMaintenancesOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .list_maintenances(ListMaintenancesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
