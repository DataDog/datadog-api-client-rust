// Create team connections returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::ConnectedTeamRef;
use datadog_api_client::datadogV2::model::ConnectedTeamRefData;
use datadog_api_client::datadogV2::model::ConnectedTeamRefDataType;
use datadog_api_client::datadogV2::model::TeamConnectionAttributes;
use datadog_api_client::datadogV2::model::TeamConnectionCreateData;
use datadog_api_client::datadogV2::model::TeamConnectionCreateRequest;
use datadog_api_client::datadogV2::model::TeamConnectionRelationships;
use datadog_api_client::datadogV2::model::TeamConnectionType;
use datadog_api_client::datadogV2::model::TeamRef;
use datadog_api_client::datadogV2::model::TeamRefData;
use datadog_api_client::datadogV2::model::TeamRefDataType;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = TeamConnectionCreateRequest::new(vec![TeamConnectionCreateData::new(
        TeamConnectionType::TEAM_CONNECTION,
    )
    .attributes(
        TeamConnectionAttributes::new()
            .managed_by("datadog".to_string())
            .source("github".to_string()),
    )
    .relationships(
        TeamConnectionRelationships::new()
            .connected_team(ConnectedTeamRef::new().data(ConnectedTeamRefData::new(
                "@MyGitHubAccount/my-team-name".to_string(),
                ConnectedTeamRefDataType::GITHUB_TEAM,
            )))
            .team(TeamRef::new().data(TeamRefData::new(
                dd_team_data_id.clone(),
                TeamRefDataType::TEAM,
            ))),
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTeamConnections", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api.create_team_connections(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
