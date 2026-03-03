// List event email addresses returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::api_events::ListEventEmailAddressesOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListEventEmailAddresses", true);
    let api = EventsAPI::with_config(configuration);
    let resp = api
        .list_event_email_addresses(ListEventEmailAddressesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
