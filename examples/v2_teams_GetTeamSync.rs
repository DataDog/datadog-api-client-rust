// Get team sync configurations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamSyncAttributesSource;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetTeamSync", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api.get_team_sync(TeamSyncAttributesSource::GITHUB).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
