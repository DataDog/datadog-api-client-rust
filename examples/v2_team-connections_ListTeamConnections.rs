// List team connections returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_team_connections::ListTeamConnectionsOptionalParams;
use datadog_api_client::datadogV2::api_team_connections::TeamConnectionsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListTeamConnections", true);
    let api = TeamConnectionsAPI::with_config(configuration);
    let resp = api
        .list_team_connections(ListTeamConnectionsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
