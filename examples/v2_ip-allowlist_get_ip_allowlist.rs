// Get IP Allowlist returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ip_allowlist::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = IPAllowlistAPI::with_config(configuration);
    let resp = api.get_ip_allowlist().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
