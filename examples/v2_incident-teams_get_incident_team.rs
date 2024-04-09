// Get details of an incident team returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incident_teams::GetIncidentTeamOptionalParams;
use datadog_api_client::datadogV2::api_incident_teams::IncidentTeamsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "team" in the system
    let team_data_id = std::env::var("TEAM_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentTeam", true);
    let api = IncidentTeamsAPI::with_config(configuration);
    let resp = api
        .get_incident_team(
            team_data_id.clone(),
            GetIncidentTeamOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
