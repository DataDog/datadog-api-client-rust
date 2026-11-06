// List all Custom OIDC providers returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_oidc_cloud_auth_providers", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.list_oidc_cloud_auth_providers().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
