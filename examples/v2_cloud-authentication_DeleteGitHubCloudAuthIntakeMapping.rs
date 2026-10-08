// Delete a GitHub cloud auth intake mapping returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.delete_git_hub_cloud_auth_intake_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api
        .delete_git_hub_cloud_auth_intake_mapping("intake_mapping_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
