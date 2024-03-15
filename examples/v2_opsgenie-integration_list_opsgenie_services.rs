// Get all service objects returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_opsgenie_integration::OpsgenieIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = OpsgenieIntegrationAPI::with_config(configuration);
    let resp = api.list_opsgenie_services().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
