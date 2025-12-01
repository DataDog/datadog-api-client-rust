// Get team hierarchy links returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::ListTeamHierarchyLinksOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "team_hierarchy_link" in the system
    let team_hierarchy_link_data_relationships_parent_team_data_id =
        std::env::var("TEAM_HIERARCHY_LINK_DATA_RELATIONSHIPS_PARENT_TEAM_DATA_ID").unwrap();
    let team_hierarchy_link_data_relationships_sub_team_data_id =
        std::env::var("TEAM_HIERARCHY_LINK_DATA_RELATIONSHIPS_SUB_TEAM_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .list_team_hierarchy_links(
            ListTeamHierarchyLinksOptionalParams::default()
                .filter_parent_team(
                    team_hierarchy_link_data_relationships_parent_team_data_id.clone(),
                )
                .filter_sub_team(team_hierarchy_link_data_relationships_sub_team_data_id.clone())
                .page_number(0)
                .page_size(100),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
