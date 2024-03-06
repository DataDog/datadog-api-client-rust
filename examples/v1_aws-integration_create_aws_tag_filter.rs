// Set an AWS tag filter returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_aws_integration::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body = AWSTagFilterCreateRequest::new()
        .account_id("123456789012".to_string())
        .namespace(AWSNamespace::ELB)
        .tag_filter_str("prod*".to_string());
    let configuration = Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.create_aws_tag_filter(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}