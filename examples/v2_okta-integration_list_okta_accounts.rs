// List Okta accounts returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_okta_integration::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = OktaIntegrationAPI::with_config(configuration);
    let resp = api.list_okta_accounts().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}