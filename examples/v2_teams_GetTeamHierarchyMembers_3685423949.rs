// Get team hierarchy members sorts inherited members before direct members
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::GetTeamHierarchyMembersOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_team_hierarchy_members", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .get_team_hierarchy_members(
            dd_team_data_id.clone(),
            GetTeamHierarchyMembersOptionalParams::default()
                .filter_keyword("Example-Team".to_string())
                .sort("is_direct,-name".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
