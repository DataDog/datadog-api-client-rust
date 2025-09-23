// Get AWS integration standard IAM permissions returns "AWS IAM Permissions
// object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.get_aws_integration_iam_permissions_standard().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
