// Get a quick list of events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.list_events().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
