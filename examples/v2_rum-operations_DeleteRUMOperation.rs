// Delete a RUM operation returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteRUMOperation", true);
    let api = RUMOperationsAPI::with_config(configuration);
    let resp = api
        .delete_rum_operation("rum_operation_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
