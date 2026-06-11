// List users with an identity provider override returns "OK" response with
// pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_identity_providers::IdentityProvidersAPI;
use datadog_api_client::datadogV2::api_identity_providers::ListIdentityProviderUsersOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = IdentityProvidersAPI::with_config(configuration);
    let response = api.list_identity_provider_users_with_pagination(
        "00000000-0000-0000-0000-000000000001".to_string(),
        ListIdentityProviderUsersOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
