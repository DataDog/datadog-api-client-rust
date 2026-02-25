// Create an AWS cloud authentication persona mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;
use datadog_api_client::datadogV2::model::AWSCloudAuthPersonaMappingCreateAttributes;
use datadog_api_client::datadogV2::model::AWSCloudAuthPersonaMappingCreateData;
use datadog_api_client::datadogV2::model::AWSCloudAuthPersonaMappingCreateRequest;
use datadog_api_client::datadogV2::model::AWSCloudAuthPersonaMappingType;

#[tokio::main]
async fn main() {
    let body =
        AWSCloudAuthPersonaMappingCreateRequest::new(AWSCloudAuthPersonaMappingCreateData::new(
            AWSCloudAuthPersonaMappingCreateAttributes::new(
                "test@test.com".to_string(),
                "arn:aws:iam::123456789012:user/testuser".to_string(),
            ),
            AWSCloudAuthPersonaMappingType::AWS_CLOUD_AUTH_CONFIG,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAWSCloudAuthPersonaMapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.create_aws_cloud_auth_persona_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
