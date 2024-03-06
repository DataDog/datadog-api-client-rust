// Get all powerpacks returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_powerpack::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = PowerpackAPI::with_config(configuration);
    let resp = api.list_powerpacks(ListPowerpacksOptionalParams::default().page_limit(2)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
