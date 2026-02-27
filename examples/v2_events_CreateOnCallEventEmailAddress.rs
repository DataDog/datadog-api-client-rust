// Create an on-call event email address returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::EventEmailAddressAlertType;
use datadog_api_client::datadogV2::model::EventEmailAddressFormat;
use datadog_api_client::datadogV2::model::EventEmailAddressResourceType;
use datadog_api_client::datadogV2::model::OnCallEventEmailAddressCreateAttributes;
use datadog_api_client::datadogV2::model::OnCallEventEmailAddressCreateData;
use datadog_api_client::datadogV2::model::OnCallEventEmailAddressCreateRequest;

#[tokio::main]
async fn main() {
    let body = OnCallEventEmailAddressCreateRequest::new(
        OnCallEventEmailAddressCreateData::new(
            OnCallEventEmailAddressCreateAttributes::new(
                EventEmailAddressFormat::JSON,
                "my-team".to_string(),
            )
            .alert_type(Some(EventEmailAddressAlertType::INFO))
            .description("On-call email address for my team.".to_string())
            .tags(vec![
                "env:production".to_string(),
                "team:my-team".to_string(),
            ]),
        )
        .type_(EventEmailAddressResourceType::EVENT_EMAILS),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateOnCallEventEmailAddress", true);
    let api = EventsAPI::with_config(configuration);
    let resp = api.create_on_call_event_email_address(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
