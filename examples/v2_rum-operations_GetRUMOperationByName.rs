// Get a RUM operation by name returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "rum_operation" in the system
    let rum_operation_data_attributes_name =
        std::env::var("RUM_OPERATION_DATA_ATTRIBUTES_NAME").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .get_rum_operation_by_name(rum_operation_data_attributes_name.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
