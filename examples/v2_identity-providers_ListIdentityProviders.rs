// List identity providers returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_identity_providers::IdentityProvidersAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = IdentityProvidersAPI::with_config(configuration);
    let resp = api.list_identity_providers().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
