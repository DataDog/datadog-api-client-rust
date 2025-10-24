// Get on-call event email address returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetOnCallEventEmailAddress", true);
    let api = EventsAPI::with_config(configuration);
    let resp = api.get_on_call_event_email_address().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
