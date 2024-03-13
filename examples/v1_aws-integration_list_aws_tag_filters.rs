// Get all AWS tag filters returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_integration::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.list_aws_tag_filters("account_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}