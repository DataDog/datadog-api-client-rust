// List all Azure integrations returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_azure_integration::AzureIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = AzureIntegrationAPI::with_config(configuration);
    let resp = api.list_azure_integration().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
