// Create a team hierarchy link returns "CREATED" response
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
    let body = TeamHierarchyLinkCreateRequest::new(TeamHierarchyLinkCreate::new(
        TeamHierarchyLinkCreateRelationships::new(
            TeamHierarchyLinkCreateTeamRelationship::new(TeamHierarchyLinkCreateTeam::new(
                "692e8073-12c4-4c71-8408-5090bd44c9c8".to_string(),
                TeamType::TEAM,
            )),
            TeamHierarchyLinkCreateTeamRelationship::new(TeamHierarchyLinkCreateTeam::new(
                "692e8073-12c4-4c71-8408-5090bd44c9c8".to_string(),
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
