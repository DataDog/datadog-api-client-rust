// Preview AWS metric name filter returns "AWS metric name filter preview result"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSMetricNameFilterPreviewRequest;
use datadog_api_client::datadogV2::model::AWSMetricNameFilterPreviewRequestAttributes;
use datadog_api_client::datadogV2::model::AWSMetricNameFilterPreviewRequestData;
use datadog_api_client::datadogV2::model::AWSMetricNameFilterPreviewType;
use datadog_api_client::datadogV2::model::AWSMetricNameFilters;
use datadog_api_client::datadogV2::model::AWSMetricNameFiltersIncludeOnly;

#[tokio::main]
async fn main() {
    let body = AWSMetricNameFilterPreviewRequest::new(AWSMetricNameFilterPreviewRequestData::new(
        AWSMetricNameFilterPreviewRequestAttributes::new(vec![
            AWSMetricNameFilters::AWSMetricNameFiltersIncludeOnly(Box::new(
                AWSMetricNameFiltersIncludeOnly::new(
                    vec!["aws.ec2.network_in".to_string()],
                    "AWS/EC2".to_string(),
                ),
            )),
        ]),
        AWSMetricNameFilterPreviewType::METRIC_NAME_FILTER_PREVIEW,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PreviewAWSMetricNameFilter", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .preview_aws_metric_name_filter("aws_account_config_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
