// Delete a user profile returns "Deletion accepted; the profile will be removed
// asynchronously" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_user_profile", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.delete_user_profile("user-1234".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
