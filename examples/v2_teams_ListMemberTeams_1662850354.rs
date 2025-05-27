// Get all member teams returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::ListMemberTeamsOptionalParams;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListMemberTeams", true);
    let api = TeamsAPI::with_config(configuration);
    let response = api.list_member_teams_with_pagination(
        "super_team_id".to_string(),
        ListMemberTeamsOptionalParams::default(),
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
