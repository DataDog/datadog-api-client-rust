// Get details of an incident team returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_teams::IncidentTeamsAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "team" in the system
    let team_data_id = std::env::var("TEAM_DATA_ID").unwrap();
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentTeam", true);
    let api = IncidentTeamsAPI::with_config(configuration);
    let resp = api.get_incident_team().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
