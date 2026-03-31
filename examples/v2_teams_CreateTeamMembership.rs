// Add a user to a team returns "Represents a user's association to a team"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::RelationshipToUserTeamUser;
use datadog_api_client::datadogV2::model::RelationshipToUserTeamUserData;
use datadog_api_client::datadogV2::model::UserTeamAttributes;
use datadog_api_client::datadogV2::model::UserTeamCreate;
use datadog_api_client::datadogV2::model::UserTeamRelationships;
use datadog_api_client::datadogV2::model::UserTeamRequest;
use datadog_api_client::datadogV2::model::UserTeamRole;
use datadog_api_client::datadogV2::model::UserTeamType;
use datadog_api_client::datadogV2::model::UserTeamUserType;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();

    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = UserTeamRequest::new(
        UserTeamCreate::new(UserTeamType::TEAM_MEMBERSHIPS)
            .attributes(UserTeamAttributes::new().role(Some(UserTeamRole::ADMIN)))
            .relationships(
                UserTeamRelationships::new().user(RelationshipToUserTeamUser::new(
                    RelationshipToUserTeamUserData::new(
                        user_data_id.clone(),
                        UserTeamUserType::USERS,
                    ),
                )),
            ),
    );
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .create_team_membership(dd_team_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
