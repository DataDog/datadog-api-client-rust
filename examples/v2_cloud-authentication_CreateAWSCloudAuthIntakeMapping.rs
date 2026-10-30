// Create an AWS cloud authentication intake mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;
use datadog_api_client::datadogV2::model::AWSCloudAuthIntakeMappingCreateAttributes;
use datadog_api_client::datadogV2::model::AWSCloudAuthIntakeMappingCreateData;
use datadog_api_client::datadogV2::model::AWSCloudAuthIntakeMappingCreateRequest;
use datadog_api_client::datadogV2::model::AWSCloudAuthIntakeMappingType;

#[tokio::main]
async fn main() {
    // there is an integrated AWS test account for WIF
    let body =
        AWSCloudAuthIntakeMappingCreateRequest::new(AWSCloudAuthIntakeMappingCreateData::new(
            AWSCloudAuthIntakeMappingCreateAttributes::new(
                "arn:aws:sts::123456789012:assumed-role/DatadogAgent-de20b7e944abfac3/*"
                    .to_string(),
            ),
            AWSCloudAuthIntakeMappingType::AWS_CLOUD_AUTH_INTAKE_MAPPING,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_aws_cloud_auth_intake_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.create_aws_cloud_auth_intake_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
