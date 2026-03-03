// Create AWS CCM config returns "AWS CCM Config object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSCcmConfig;
use datadog_api_client::datadogV2::model::AWSCcmConfigRequest;
use datadog_api_client::datadogV2::model::AWSCcmConfigRequestAttributes;
use datadog_api_client::datadogV2::model::AWSCcmConfigRequestData;
use datadog_api_client::datadogV2::model::AWSCcmConfigType;
use datadog_api_client::datadogV2::model::DataExportConfig;

#[tokio::main]
async fn main() {
    let body = AWSCcmConfigRequest::new(AWSCcmConfigRequestData::new(
        AWSCcmConfigRequestAttributes::new(AWSCcmConfig::new(vec![DataExportConfig::new(
            "billing".to_string(),
            "us-east-1".to_string(),
            "cost-and-usage-report".to_string(),
            "reports".to_string(),
            "CUR2.0".to_string(),
        )])),
        AWSCcmConfigType::CCM_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAWSAccountCCMConfig", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .create_aws_account_ccm_config("aws_account_config_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
