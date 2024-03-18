// Create a new incident team returns "CREATED" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_teams::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = IncidentTeamCreateRequest::new(
        IncidentTeamCreateData::new(IncidentTeamType::TEAMS).attributes(
            IncidentTeamCreateAttributes::new("Example-Incident-Team".to_string()),
        ),
    );
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentTeam", true);
    let api = IncidentTeamsAPI::with_config(configuration);
    let resp = api.create_incident_team(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
