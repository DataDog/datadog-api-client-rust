// List timestamp overrides for an incident returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::ListIncidentTimestampOverridesOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidentTimestampOverrides", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .list_incident_timestamp_overrides(
            Uuid::parse_str("9cecfde8-e35d-4387-8985-9b30dcb9cb1c").expect("invalid UUID"),
            ListIncidentTimestampOverridesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
