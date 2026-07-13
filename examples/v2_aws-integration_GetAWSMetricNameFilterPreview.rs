// Get AWS metric name filter preview returns "AWS metric name filter preview
// result" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetAWSMetricNameFilterPreview", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .get_aws_metric_name_filter_preview("aws_account_config_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
