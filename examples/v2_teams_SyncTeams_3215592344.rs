// Sync teams returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamSyncAttributes;
use datadog_api_client::datadogV2::model::TeamSyncAttributesSource;
use datadog_api_client::datadogV2::model::TeamSyncAttributesType;
use datadog_api_client::datadogV2::model::TeamSyncBulkType;
use datadog_api_client::datadogV2::model::TeamSyncData;
use datadog_api_client::datadogV2::model::TeamSyncRequest;

#[tokio::main]
async fn main() {
    let body = TeamSyncRequest::new(TeamSyncData::new(
        TeamSyncAttributes::new(
            TeamSyncAttributesSource::GITHUB,
            TeamSyncAttributesType::LINK,
        ),
        TeamSyncBulkType::TEAM_SYNC_BULK,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SyncTeams", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api.sync_teams(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
