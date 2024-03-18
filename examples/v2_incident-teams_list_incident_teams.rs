// Get a list of all incident teams returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_teams::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "team" in the system
    let team_data_attributes_name = std::env::var("TEAM_DATA_ATTRIBUTES_NAME").unwrap();
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidentTeams", true);
    let api = IncidentTeamsAPI::with_config(configuration);
    let resp = api
        .list_incident_teams(
            ListIncidentTeamsOptionalParams::default().filter(team_data_attributes_name.clone()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
