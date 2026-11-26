// List RUM operation links returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::ListRUMOperationStrongLinksOptionalParams;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "rum_operation" in the system
    let rum_operation_data_id = std::env::var("RUM_OPERATION_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .list_rum_operation_strong_links(
            ListRUMOperationStrongLinksOptionalParams::default()
                .operation_id(rum_operation_data_id.clone()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
