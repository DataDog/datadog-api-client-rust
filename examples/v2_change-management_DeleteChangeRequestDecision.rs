// Delete a change request decision returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_change_management::ChangeManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteChangeRequestDecision", true);
    let api = ChangeManagementAPI::with_config(configuration);
    let resp = api
        .delete_change_request_decision("change_request_id".to_string(), "decision_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
