// Get all member teams returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::ListMemberTeamsOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListMemberTeams", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .list_member_teams(
            "super_team_id".to_string(),
            ListMemberTeamsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
