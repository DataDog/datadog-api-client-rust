// Update a team returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_attributes_handle = std::env::var("DD_TEAM_DATA_ATTRIBUTES_HANDLE").unwrap();
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = TeamUpdateRequest::new(TeamUpdate::new(
        TeamUpdateAttributes::new(
            dd_team_data_attributes_handle.clone(),
            "Example Team updated".to_string(),
        )
        .avatar(Some("🥑".to_string()))
        .banner(Some(7))
        .hidden_modules(vec!["m3".to_string()])
        .visible_modules(vec!["m1".to_string(), "m2".to_string()]),
        TeamType::TEAM,
    ));
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api.update_team(dd_team_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
