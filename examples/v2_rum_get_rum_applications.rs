// List all the RUM applications returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_rum::RUMAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api.get_rum_applications().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
