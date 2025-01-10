// Get an AWS integration by config ID returns "AWS Account object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "aws_account_v2" in the system
    let aws_account_v2_data_id = std::env::var("AWS_ACCOUNT_V2_DATA_ID").unwrap();

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetAWSAccount", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.get_aws_account(aws_account_v2_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
