// Get team hierarchy links returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::GetTeamHierarchyLinksOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .get_team_hierarchy_links(GetTeamHierarchyLinksOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
