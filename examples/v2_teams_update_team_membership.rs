// Update a user's membership attributes on a team returns "Represents a user's
// association to a team" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = UserTeamUpdateRequest::new(
        UserTeamUpdate::new(UserTeamType::TEAM_MEMBERSHIPS)
            .attributes(UserTeamAttributes::new().role(Some(UserTeamRole::ADMIN))),
    );
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .update_team_membership("team_id".to_string(), "user_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
