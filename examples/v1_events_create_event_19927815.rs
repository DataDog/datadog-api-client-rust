// Post an event with a long title returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_events::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        EventCreateRequest::new(
            "A text message.".to_string(),
            "Example-Event very very very looooooooong looooooooooooong loooooooooooooooooooooong looooooooooooooooooooooooooong title with 100+ characters".to_string(),
        ).tags(vec!["test:ExampleEvent".to_string()]);
    let configuration = Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.create_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
