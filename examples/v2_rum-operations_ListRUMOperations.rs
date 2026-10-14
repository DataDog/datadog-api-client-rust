// Search RUM operations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::ListRUMOperationsOptionalParams;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .list_rum_operations(ListRUMOperationsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
