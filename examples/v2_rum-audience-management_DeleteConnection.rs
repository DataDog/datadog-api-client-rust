// Delete connection returns "Connection deleted successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteConnection", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api
        .delete_connection("connection-id-123".to_string(), "users".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
