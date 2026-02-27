// Revoke an on-call event email address returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteOnCallEventEmailAddress", true);
    let api = EventsAPI::with_config(configuration);
    let resp = api
        .delete_on_call_event_email_address(
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
