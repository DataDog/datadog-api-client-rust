// Validate AWS CCM config returns "AWS CCM Config validation result" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSCcmConfigValidationRequest;
use datadog_api_client::datadogV2::model::AWSCcmConfigValidationRequestAttributes;
use datadog_api_client::datadogV2::model::AWSCcmConfigValidationRequestData;
use datadog_api_client::datadogV2::model::AWSCcmConfigValidationType;

#[tokio::main]
async fn main() {
    let body = AWSCcmConfigValidationRequest::new(AWSCcmConfigValidationRequestData::new(
        AWSCcmConfigValidationRequestAttributes::new(
            "123456789012".to_string(),
            "billing".to_string(),
            "us-east-1".to_string(),
            "cost-and-usage-report".to_string(),
        )
        .report_prefix("reports".to_string()),
        AWSCcmConfigValidationType::CCM_CONFIG_VALIDATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ValidateAWSCCMConfig", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.validate_awsccm_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
