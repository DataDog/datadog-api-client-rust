// List namespaces returns "AWS Namespaces List object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.list_aws_namespaces().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
