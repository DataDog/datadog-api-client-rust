// Add AWS Log Lambda ARN returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_logs_integration::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = AWSAccountAndLambdaRequest::new(
        "1234567".to_string(),
        "arn:aws:lambda:us-east-1:1234567:function:LogsCollectionAPITest".to_string(),
    );
    let configuration = Configuration::new();
    let api = AWSLogsIntegrationAPI::with_config(configuration);
    let resp = api.create_aws_lambda_arn(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
