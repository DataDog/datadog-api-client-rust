// Enable an AWS Logs integration returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_logs_integration::AWSLogsIntegrationAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        AWSLogsServicesRequest::new(
            "1234567".to_string(),
            vec![
                "s3".to_string(),
                "elb".to_string(),
                "elbv2".to_string(),
                "cloudfront".to_string(),
                "redshift".to_string(),
                "lambda".to_string()
            ],
        );
    let configuration = Configuration::new();
    let api = AWSLogsIntegrationAPI::with_config(configuration);
    let resp = api.enable_aws_log_services(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
