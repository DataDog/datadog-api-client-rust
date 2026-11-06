// Create a Custom OIDC persona mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;
use datadog_api_client::datadogV2::model::OIDCCloudAuthClaimMatchMode;
use datadog_api_client::datadogV2::model::OIDCCloudAuthClaimMatcherCreate;
use datadog_api_client::datadogV2::model::OIDCCloudAuthPersonaMappingCreateAttributes;
use datadog_api_client::datadogV2::model::OIDCCloudAuthPersonaMappingCreateData;
use datadog_api_client::datadogV2::model::OIDCCloudAuthPersonaMappingCreateRequest;
use datadog_api_client::datadogV2::model::OIDCCloudAuthPersonaMappingType;

#[tokio::main]
async fn main() {
    // there is a valid "oidc_cloud_auth_provider" in the system
    let oidc_cloud_auth_provider_data_id =
        uuid::Uuid::parse_str(&std::env::var("OIDC_CLOUD_AUTH_PROVIDER_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "service_account_user_with_role" in the system
    let service_account_user_with_role_data_id =
        std::env::var("SERVICE_ACCOUNT_USER_WITH_ROLE_DATA_ID").unwrap();
    let body =
        OIDCCloudAuthPersonaMappingCreateRequest::new(OIDCCloudAuthPersonaMappingCreateData::new(
            OIDCCloudAuthPersonaMappingCreateAttributes::new(
                service_account_user_with_role_data_id.clone(),
                oidc_cloud_auth_provider_data_id.clone(),
            )
            .claim_matchers(vec![OIDCCloudAuthClaimMatcherCreate::new(
                "sub".to_string(),
                vec!["system:serviceaccount:controllers:example-controller".to_string()],
            )
            .mode(OIDCCloudAuthClaimMatchMode::EXACT)]),
            OIDCCloudAuthPersonaMappingType::CUSTOM_OIDC_IDENTITY_MAPPING,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_oidc_cloud_auth_persona_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.create_oidc_cloud_auth_persona_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
