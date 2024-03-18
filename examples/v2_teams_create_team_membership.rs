// Add a user to a team returns "Represents a user's association to a team"
// response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = UserTeamRequest::new(
        UserTeamCreate::new(UserTeamType::TEAM_MEMBERSHIPS)
            .attributes(UserTeamAttributes::new().role(Some(UserTeamRole::ADMIN)))
            .relationships(
                UserTeamRelationships::new()
                    .team(RelationshipToUserTeamTeam::new(
                        RelationshipToUserTeamTeamData::new(
                            "d7e15d9d-d346-43da-81d8-3d9e71d9a5e9".to_string(),
                            UserTeamTeamType::TEAM,
                        ),
                    ))
                    .user(RelationshipToUserTeamUser::new(
                        RelationshipToUserTeamUserData::new(
                            "b8626d7e-cedd-11eb-abf5-da7ad0900001".to_string(),
                            UserTeamUserType::USERS,
                        ),
                    )),
            ),
    );
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .create_team_membership("team_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
