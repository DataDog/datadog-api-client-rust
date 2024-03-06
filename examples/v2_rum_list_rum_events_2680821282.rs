// Get a list of RUM events returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_rum::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api.list_rum_events(ListRUMEventsOptionalParams::default().page_limit(2)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
