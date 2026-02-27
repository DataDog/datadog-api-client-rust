// Update an event email address returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::EventEmailAddressAlertType;
use datadog_api_client::datadogV2::model::EventEmailAddressResourceType;
use datadog_api_client::datadogV2::model::EventEmailAddressUpdateAttributes;
use datadog_api_client::datadogV2::model::EventEmailAddressUpdateData;
use datadog_api_client::datadogV2::model::EventEmailAddressUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = EventEmailAddressUpdateRequest::new(
        EventEmailAddressUpdateData::new(
            EventEmailAddressUpdateAttributes::new()
                .alert_type(Some(EventEmailAddressAlertType::INFO))
                .description(Some(
                    "Updated description for the email address.".to_string(),
                ))
                .notify_handles(vec!["@slack-my-channel".to_string()])
                .tags(vec![
                    "env:production".to_string(),
                    "team:my-team".to_string(),
                ]),
        )
        .type_(EventEmailAddressResourceType::EVENT_EMAILS),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateEventEmailAddress", true);
    let api = EventsAPI::with_config(configuration);
    let resp = api
        .update_event_email_address(
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
