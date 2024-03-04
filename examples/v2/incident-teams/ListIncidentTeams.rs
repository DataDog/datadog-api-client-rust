// Get a list of all incident teams returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_teams::IncidentTeamsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "team" in the system
    let team_data_attributes_name = std::env::var("TEAM_DATA_ATTRIBUTES_NAME").unwrap();
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidentTeams", true);
    let api = IncidentTeamsAPI::with_config(configuration);
    let resp = api.list_incident_teams().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
