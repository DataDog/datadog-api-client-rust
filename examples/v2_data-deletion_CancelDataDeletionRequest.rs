// Cancels a data deletion request returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_deletion::DataDeletionAPI;

#[tokio::main]
async fn main() {
    // there is a valid "deletion_request" in the system
    let deletion_request_data_id = std::env::var("DELETION_REQUEST_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CancelDataDeletionRequest", true);
    let api = DataDeletionAPI::with_config(configuration);
    let resp = api
        .cancel_data_deletion_request(deletion_request_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
