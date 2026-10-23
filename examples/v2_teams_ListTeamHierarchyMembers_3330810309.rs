// Get team hierarchy members includes inherited users with null root membership
// attributes
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::ListTeamHierarchyMembersOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_team_hierarchy_members", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .list_team_hierarchy_members(
            dd_team_data_id.clone(),
            ListTeamHierarchyMembersOptionalParams::default()
                .filter_keyword("Example-Team".to_string())
                .include("user,team".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
