// Create an HCP Terraform persona mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;
use datadog_api_client::datadogV2::model::TerraformCloudAuthIssuerURL;
use datadog_api_client::datadogV2::model::TerraformCloudAuthPersonaMappingCreateAttributes;
use datadog_api_client::datadogV2::model::TerraformCloudAuthPersonaMappingCreateData;
use datadog_api_client::datadogV2::model::TerraformCloudAuthPersonaMappingCreateRequest;
use datadog_api_client::datadogV2::model::TerraformCloudAuthPersonaMappingType;

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user_with_role" in the system
    let service_account_user_with_role_data_attributes_email =
        std::env::var("SERVICE_ACCOUNT_USER_WITH_ROLE_DATA_ATTRIBUTES_EMAIL").unwrap();
    let body =
        TerraformCloudAuthPersonaMappingCreateRequest::new(
            TerraformCloudAuthPersonaMappingCreateData::new(
                TerraformCloudAuthPersonaMappingCreateAttributes::new(
                    service_account_user_with_role_data_attributes_email.clone(),
                    TerraformCloudAuthIssuerURL::US,
                    "organization:datadog-api-spec:project:Default Project:workspace:sdk-examplecloudauthentication:run_phase:*".to_string(),
                ),
                TerraformCloudAuthPersonaMappingType::TERRAFORM_CLOUD_AUTH_CONFIG,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_terraform_cloud_auth_persona_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.create_terraform_cloud_auth_persona_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
