// AWS Integration - Delete account returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteAWSAccountv2", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .delete_aws_accountv2("aws_account_config_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
