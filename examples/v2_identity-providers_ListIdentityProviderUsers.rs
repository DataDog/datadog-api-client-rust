// List users with an identity provider override returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_identity_providers::IdentityProvidersAPI;
use datadog_api_client::datadogV2::api_identity_providers::ListIdentityProviderUsersOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = IdentityProvidersAPI::with_config(configuration);
    let resp = api
        .list_identity_provider_users(
            "00000000-0000-0000-0000-000000000001".to_string(),
            ListIdentityProviderUsersOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
