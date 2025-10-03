// List tables returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ListTablesOptionalParams;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.list_tables(ListTablesOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
