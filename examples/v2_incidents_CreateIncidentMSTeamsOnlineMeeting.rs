// Create an incident Microsoft Teams online meeting returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentMSTeamsOnlineMeeting", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_ms_teams_online_meeting("incident_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
