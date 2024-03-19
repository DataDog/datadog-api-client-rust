// Get an event returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_events::EventsAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.get_event(9223372036854775807).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
