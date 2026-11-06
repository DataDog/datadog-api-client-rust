// Create a Custom OIDC provider returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;
use datadog_api_client::datadogV2::model::OIDCCloudAuthDiscoveryType;
use datadog_api_client::datadogV2::model::OIDCCloudAuthProviderCreateAttributes;
use datadog_api_client::datadogV2::model::OIDCCloudAuthProviderCreateData;
use datadog_api_client::datadogV2::model::OIDCCloudAuthProviderCreateRequest;
use datadog_api_client::datadogV2::model::OIDCCloudAuthProviderType;

#[tokio::main]
async fn main() {
    let body = OIDCCloudAuthProviderCreateRequest::new(OIDCCloudAuthProviderCreateData::new(
        OIDCCloudAuthProviderCreateAttributes::new(
            vec!["sdk-test".to_string()],
            OIDCCloudAuthDiscoveryType::OIDC_DISCOVERY,
            "https://oidc-de20b7e944abfac3.example.com".to_string(),
            "oidc-de20b7e944abfac3".to_string(),
        ),
        OIDCCloudAuthProviderType::CUSTOM_OIDC_PROVIDER_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_oidc_cloud_auth_provider", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.create_oidc_cloud_auth_provider(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
