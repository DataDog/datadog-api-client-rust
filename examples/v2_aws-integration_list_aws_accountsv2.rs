// AWS Integration - Get all accounts returns "AWS Account object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListAWSAccountsv2", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.list_aws_accountsv2().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
