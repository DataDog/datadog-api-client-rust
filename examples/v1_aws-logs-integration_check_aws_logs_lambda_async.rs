// Check that an AWS Lambda Function exists returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_logs_integration::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        AWSAccountAndLambdaRequest::new(
            "1234567".to_string(),
            "arn:aws:lambda:us-east-1:1234567:function:LogsCollectionAPITest".to_string(),
        );
    let configuration = Configuration::new();
    let api = AWSLogsIntegrationAPI::with_config(configuration);
    let resp = api.check_aws_logs_lambda_async(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
