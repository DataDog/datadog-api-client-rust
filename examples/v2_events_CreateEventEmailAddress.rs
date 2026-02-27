// Create an event email address returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::EventEmailAddressAlertType;
use datadog_api_client::datadogV2::model::EventEmailAddressCreateAttributes;
use datadog_api_client::datadogV2::model::EventEmailAddressCreateData;
use datadog_api_client::datadogV2::model::EventEmailAddressCreateRequest;
use datadog_api_client::datadogV2::model::EventEmailAddressFormat;
use datadog_api_client::datadogV2::model::EventEmailAddressResourceType;

#[tokio::main]
async fn main() {
    let body = EventEmailAddressCreateRequest::new(
        EventEmailAddressCreateData::new(
            EventEmailAddressCreateAttributes::new(EventEmailAddressFormat::JSON)
                .alert_type(Some(EventEmailAddressAlertType::INFO))
                .description("Email address for production alerts.".to_string())
                .notify_handles(vec!["@slack-my-channel".to_string()])
                .tags(vec![
                    "env:production".to_string(),
                    "team:my-team".to_string(),
                ]),
        )
        .type_(EventEmailAddressResourceType::EVENT_EMAILS),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateEventEmailAddress", true);
    let api = EventsAPI::with_config(configuration);
    let resp = api.create_event_email_address(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
