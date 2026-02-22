// List personal access tokens returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV2::api_key_management::ListPersonalAccessTokensOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListPersonalAccessTokens", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .list_personal_access_tokens(ListPersonalAccessTokensOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
