// Create a team hierarchy link returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamHierarchyLinkCreate;
use datadog_api_client::datadogV2::model::TeamHierarchyLinkCreateRelationships;
use datadog_api_client::datadogV2::model::TeamHierarchyLinkCreateRequest;
use datadog_api_client::datadogV2::model::TeamHierarchyLinkCreateTeam;
use datadog_api_client::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship;
use datadog_api_client::datadogV2::model::TeamHierarchyLinkType;
use datadog_api_client::datadogV2::model::TeamType;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();

    // there is a valid "dd_team_2" in the system
    let dd_team_2_data_id = std::env::var("DD_TEAM_2_DATA_ID").unwrap();
    let body = TeamHierarchyLinkCreateRequest::new(TeamHierarchyLinkCreate::new(
        TeamHierarchyLinkCreateRelationships::new(
            TeamHierarchyLinkCreateTeamRelationship::new(TeamHierarchyLinkCreateTeam::new(
                dd_team_data_id.clone(),
                TeamType::TEAM,
            )),
            TeamHierarchyLinkCreateTeamRelationship::new(TeamHierarchyLinkCreateTeam::new(
                dd_team_2_data_id.clone(),
                TeamType::TEAM,
            )),
        ),
        TeamHierarchyLinkType::TEAM_HIERARCHY_LINKS,
    ));
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api.add_team_hierarchy_link(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
