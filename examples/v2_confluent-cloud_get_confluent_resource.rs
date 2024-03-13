// Get resource from Confluent account returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_confluent_cloud::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ConfluentCloudAPI::with_config(configuration);
    let resp = api
        .get_confluent_resource("account_id".to_string(), "resource_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
