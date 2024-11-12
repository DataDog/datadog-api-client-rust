// Get list of AWS log ready services returns "AWS Logs Services List object"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_logs_integration::AWSLogsIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListAWSLogsServices", true);
    let api = AWSLogsIntegrationAPI::with_config(configuration);
    let resp = api.list_aws_logs_services().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
