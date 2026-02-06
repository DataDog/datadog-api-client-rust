// Delete a timestamp override for an incident returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentTimestampOverride", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_timestamp_override(
            Uuid::parse_str("9cecfde8-e35d-4387-8985-9b30dcb9cb1c").expect("invalid UUID"),
            Uuid::parse_str("6f48a86f-9a39-4bcf-a76b-9a1b20188995").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
