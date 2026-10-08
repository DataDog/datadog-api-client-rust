// List GitHub cloud authentication intake mappings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.list_git_hub_cloud_auth_intake_mappings", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.list_git_hub_cloud_auth_intake_mappings().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
