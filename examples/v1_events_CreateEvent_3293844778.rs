// Post an event returns "OK" response with an int64 ID above 2^53
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_events::EventsAPI;
use datadog_api_client::datadogV1::model::EventCreateRequest;

#[tokio::main]
async fn main() {
    let body = EventCreateRequest::new(
        "An event whose ID exceeds Number.MAX_SAFE_INTEGER.".to_string(),
        "Example-Event".to_string(),
    )
    .tags(vec!["test:ExampleEvent".to_string()]);
    let configuration = datadog::Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.create_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
