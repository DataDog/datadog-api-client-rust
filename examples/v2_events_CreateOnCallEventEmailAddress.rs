// Create on-call event email address returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::CreateOnCallEventEmailAddressRequest;
use datadog_api_client::datadogV2::model::CreateOnCallEventEmailAddressRequestData;
use datadog_api_client::datadogV2::model::CreateOnCallEventEmailAddressRequestDataAttributes;
use datadog_api_client::datadogV2::model::EventEmailsType;

#[tokio::main]
async fn main() {
    let body = CreateOnCallEventEmailAddressRequest::new().data(
        CreateOnCallEventEmailAddressRequestData::new(EventEmailsType::EVENT_EMAILS).attributes(
            CreateOnCallEventEmailAddressRequestDataAttributes::new(
                "".to_string(),
                vec!["".to_string()],
            ),
        ),
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
