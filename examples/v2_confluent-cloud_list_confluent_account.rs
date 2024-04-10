// List Confluent accounts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_confluent_cloud::ConfluentCloudAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ConfluentCloudAPI::with_config(configuration);
    let resp = api.list_confluent_account().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
