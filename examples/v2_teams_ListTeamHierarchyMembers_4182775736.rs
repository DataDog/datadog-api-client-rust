// Get team hierarchy members returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::ListTeamHierarchyMembersOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_team_hierarchy_members", true);
    let api = TeamsAPI::with_config(configuration);
    let response = api.list_team_hierarchy_members_with_pagination(
        dd_team_data_id.clone(),
        ListTeamHierarchyMembersOptionalParams::default()
            .filter_keyword("Example-Team".to_string())
            .page_size(1),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
