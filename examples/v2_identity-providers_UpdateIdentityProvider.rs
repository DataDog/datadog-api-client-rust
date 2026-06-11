// Update an identity provider returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_identity_providers::IdentityProvidersAPI;
use datadog_api_client::datadogV2::model::IdentityProviderType;
use datadog_api_client::datadogV2::model::IdentityProviderUpdateAttributes;
use datadog_api_client::datadogV2::model::IdentityProviderUpdateData;
use datadog_api_client::datadogV2::model::IdentityProviderUpdateRequest;

#[tokio::main]
async fn main() {
    let body = IdentityProviderUpdateRequest::new(IdentityProviderUpdateData::new(
        IdentityProviderUpdateAttributes::new(true),
        "00000000-0000-0000-0000-000000000001".to_string(),
        IdentityProviderType::IDENTITY_PROVIDERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = IdentityProvidersAPI::with_config(configuration);
    let resp = api
        .update_identity_provider("00000000-0000-0000-0000-000000000001".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
